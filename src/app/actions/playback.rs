use super::*;

impl App {
    pub fn persist_playback_state(&mut self) -> Result<()> {
        self.playback_state_dirty = true;
        self.flush_playback_state(false)
    }

    pub fn persist_playback_state_now(&mut self) -> Result<()> {
        self.playback_state_dirty = true;
        self.flush_playback_state(true)
    }

    pub fn play_track(&mut self, track_id: i64) -> Result<()> {
        self.ensure_track_in_queue(track_id)?;
        self.play_track_from_position(track_id, 0, true)?;
        Ok(())
    }

    pub fn next_track(&mut self) -> Result<()> {
        let current_idx = self.current_queue_index().unwrap_or(usize::MAX);

        let Some(next_idx) = compute_next_track_index(
            self.session.queue.len(),
            current_idx,
            self.session.playback_state.repeat_mode,
        ) else {
            return Ok(());
        };

        let id = self.session.queue[next_idx];
        self.play_track(id)
    }

    pub fn prev_track(&mut self) -> Result<()> {
        let current_idx = self.current_queue_index().unwrap_or(0);

        let Some(prev_idx) = compute_prev_track_index(self.session.queue.len(), current_idx) else {
            return Ok(());
        };

        let id = self.session.queue[prev_idx];
        self.play_track(id)
    }

    pub fn seek(&mut self, delta_secs: i64) -> Result<()> {
        let next = self.player.seek_relative(delta_secs)?;
        self.session.playback_state.position_secs = next;
        self.persist_playback_state()?;
        Ok(())
    }

    pub fn toggle_play_pause(&mut self) -> Result<bool> {
        if self.player.has_active_sink() {
            let paused = self.player.toggle_pause();
            return Ok(paused);
        }

        if let Some(track_id) = self.session.playback_state.current_track_id {
            let start_at = self.session.playback_state.position_secs;
            self.play_track_from_position(track_id, start_at, false)?;
            return Ok(false);
        }

        if let Some(first_track) = self.session.tracks.first() {
            self.play_track(first_track.id)?;
            return Ok(false);
        }

        Ok(true)
    }

    pub fn refresh_playback_position(&mut self) -> Result<()> {
        self.player.poll_analysis_results();

        if self.player.consume_track_finished() {
            let previous_track = self.session.playback_state.current_track_id;
            self.next_track()?;

            if self.session.playback_state.current_track_id == previous_track
                && !self.player.has_active_sink()
            {
                self.session.playback_state.current_track_id = None;
                self.session.playback_state.position_secs = 0;
                self.persist_playback_state()?;
            }
        }

        self.session.playback_state.position_secs = self.player.current_position_secs();
        self.flush_playback_state(false)?;
        Ok(())
    }

    pub fn toggle_repeat(&mut self) -> Result<RepeatMode> {
        self.session.playback_state.repeat_mode = self.session.playback_state.repeat_mode.cycle();
        self.persist_playback_state()?;
        Ok(self.session.playback_state.repeat_mode)
    }

    pub fn adjust_volume(&mut self, delta: f32) -> u8 {
        self.player.adjust_volume(delta)
    }

    pub fn volume_percent(&self) -> u8 {
        self.player.volume_percent()
    }

    pub fn is_actively_playing(&self) -> bool {
        self.player.has_active_sink() && !self.player.is_paused()
    }

    pub fn visualizer_levels(&self, bars: usize) -> Vec<(f32, f32)> {
        self.player.visualizer_levels(bars)
    }

    pub(super) fn flush_playback_state(&mut self, force: bool) -> Result<()> {
        if !self.playback_state_dirty {
            return Ok(());
        }

        let unchanged = self
            .last_persisted_playback_state
            .as_ref()
            .is_some_and(|last| *last == self.session.playback_state);
        if unchanged {
            self.playback_state_dirty = false;
            return Ok(());
        }

        if !force
            && self
                .last_persisted_at
                .is_some_and(|at| at.elapsed() < PLAYBACK_PERSIST_DEBOUNCE)
        {
            return Ok(());
        }

        self.storage
            .save_playback_state(&self.session.playback_state)?;
        self.last_persisted_playback_state = Some(self.session.playback_state.clone());
        self.last_persisted_at = Some(std::time::Instant::now());
        self.playback_state_dirty = false;
        Ok(())
    }

    fn current_queue_index(&self) -> Option<usize> {
        self.session.playback_state.current_track_id.and_then(|id| {
            self.session
                .queue
                .iter()
                .position(|queue_id| *queue_id == id)
        })
    }

    fn play_track_from_position(
        &mut self,
        track_id: i64,
        start_at: i64,
        increment_play_count: bool,
    ) -> Result<()> {
        let Some((path, mtime)) = self
            .track_by_id(track_id)
            .map(|track| (track.path.clone(), track.mtime))
        else {
            return Ok(());
        };

        self.player.play_file(&path, mtime, start_at)?;
        self.session.playback_state.current_track_id = Some(track_id);
        self.session.playback_state.position_secs = start_at;
        if increment_play_count {
            self.storage.increment_play_count(track_id)?;
        }
        self.persist_playback_state()?;
        Ok(())
    }
}

fn compute_next_track_index(
    queue_len: usize,
    current_idx: usize,
    repeat_mode: RepeatMode,
) -> Option<usize> {
    if queue_len == 0 {
        return None;
    }
    if current_idx == usize::MAX {
        return Some(0);
    }
    if current_idx + 1 < queue_len {
        return Some(current_idx + 1);
    }
    match repeat_mode {
        RepeatMode::RepeatAll => Some(0),
        RepeatMode::RepeatOne => Some(current_idx),
        RepeatMode::Off => None,
    }
}

fn compute_prev_track_index(queue_len: usize, current_idx: usize) -> Option<usize> {
    if queue_len == 0 {
        return None;
    }
    if current_idx == 0 {
        Some(0)
    } else {
        Some(current_idx - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_next_track_index, compute_prev_track_index};
    use crate::core::model::RepeatMode;

    #[test]
    fn next_track_index_respects_repeat_modes() {
        assert_eq!(
            compute_next_track_index(3, 2, RepeatMode::Off),
            None,
            "no wrap when repeat is off"
        );
        assert_eq!(
            compute_next_track_index(3, 2, RepeatMode::RepeatAll),
            Some(0),
            "wrap when repeat all"
        );
        assert_eq!(
            compute_next_track_index(3, 2, RepeatMode::RepeatOne),
            Some(2),
            "stay on same when repeat one"
        );
    }

    #[test]
    fn prev_track_index_clamps_to_zero() {
        assert_eq!(compute_prev_track_index(3, 0), Some(0));
        assert_eq!(compute_prev_track_index(3, 2), Some(1));
        assert_eq!(compute_prev_track_index(0, 0), None);
    }
}
