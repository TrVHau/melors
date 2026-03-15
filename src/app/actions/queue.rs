use super::*;

impl App {
    pub fn toggle_shuffle(&mut self) -> Result<bool> {
        self.session.playback_state.shuffle_enabled = !self.session.playback_state.shuffle_enabled;
        self.rebuild_queue()?;
        self.persist_playback_state()?;
        Ok(self.session.playback_state.shuffle_enabled)
    }

    pub fn add_to_queue(&mut self, track_id: i64) -> Result<bool> {
        if self.track_by_id(track_id).is_none() {
            return Ok(false);
        }

        self.session.queue.push(track_id);
        self.persist_queue()?;
        Ok(true)
    }

    pub fn remove_queue_index(&mut self, index: usize) -> Result<Option<i64>> {
        if index >= self.session.queue.len() {
            return Ok(None);
        }

        let removed = self.session.queue.remove(index);
        self.persist_queue()?;
        Ok(Some(removed))
    }

    pub fn move_queue_index(&mut self, index: usize, delta: isize) -> Result<Option<usize>> {
        if index >= self.session.queue.len() {
            return Ok(None);
        }
        let next = (index as isize + delta).clamp(0, self.session.queue.len() as isize - 1);
        let next = next as usize;
        if next == index {
            return Ok(Some(index));
        }
        self.session.queue.swap(index, next);
        self.persist_queue()?;
        Ok(Some(next))
    }

    pub fn play_queue_index(&mut self, index: usize) -> Result<Option<i64>> {
        if let Some(track_id) = self.session.queue.get(index).copied() {
            self.play_track(track_id)?;
            return Ok(Some(track_id));
        }

        Ok(None)
    }

    pub fn queue_len(&self) -> usize {
        self.session.queue.len()
    }

    pub fn queue_ids(&self) -> &[i64] {
        &self.session.queue
    }

    pub fn queue_version(&self) -> u64 {
        self.session.queue_version
    }

    pub(super) fn rebuild_queue(&mut self) -> Result<()> {
        let mut ids: Vec<i64> = self.session.tracks.iter().map(|t| t.id).collect();
        if self.session.playback_state.shuffle_enabled {
            shuffle_vec(&mut ids, self.session.playback_state.current_track_id);
        }
        self.session.queue = ids;
        self.persist_queue()?;
        Ok(())
    }

    pub(super) fn normalize_queue(&mut self) -> Result<()> {
        let valid_ids: HashSet<i64> = self.session.tracks.iter().map(|track| track.id).collect();
        let original_len = self.session.queue.len();
        self.session.queue.retain(|id| valid_ids.contains(id));

        if self.session.queue.is_empty() && !self.session.tracks.is_empty() {
            self.rebuild_queue()?;
            return Ok(());
        }

        if self.session.queue.len() != original_len {
            self.persist_queue()?;
        }

        Ok(())
    }

    pub(super) fn ensure_track_in_queue(&mut self, track_id: i64) -> Result<()> {
        if self.session.queue.contains(&track_id) {
            return Ok(());
        }

        self.session.queue.push(track_id);
        self.persist_queue()?;
        Ok(())
    }

    pub(super) fn persist_queue(&mut self) -> Result<()> {
        self.storage.replace_queue(&self.session.queue)?;
        self.session.queue_version = self.session.queue_version.saturating_add(1);
        Ok(())
    }
}
