use super::*;

impl App {
    pub fn toggle_shuffle(&mut self) -> Result<bool> {
        self.clear_active_playlist_context();
        self.session.playback_state.shuffle_enabled = !self.session.playback_state.shuffle_enabled;
        self.rebuild_queue()?;
        self.persist_playback_state()?;
        Ok(self.session.playback_state.shuffle_enabled)
    }

    pub fn queue_ids(&self) -> &[i64] {
        &self.session.queue
    }

    pub(super) fn rebuild_queue(&mut self) -> Result<()> {
        self.clear_active_playlist_context();
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

        self.clear_active_playlist_context();
        self.session.queue.push(track_id);
        self.persist_queue()?;
        Ok(())
    }

    pub(super) fn activate_playlist_queue(
        &mut self,
        playlist_id: i64,
        playlist_name: String,
    ) -> Result<Vec<i64>> {
        let items = self.storage.load_playlist_items(playlist_id)?;
        let track_ids: Vec<i64> = items
            .into_iter()
            .filter(|item| !item.is_missing)
            .filter_map(|item| item.track_id)
            .filter(|track_id| self.track_by_id(*track_id).is_some())
            .collect();

        self.session.queue = track_ids.clone();
        self.session.active_playlist_id = Some(playlist_id);
        self.session.active_playlist_name = Some(playlist_name);
        self.persist_queue()?;
        Ok(track_ids)
    }

    pub(super) fn sync_active_playlist_queue_if_needed(&mut self, playlist_id: i64) -> Result<()> {
        if self.session.active_playlist_id != Some(playlist_id) {
            return Ok(());
        }

        let playlist_name = self
            .session
            .active_playlist_name
            .clone()
            .unwrap_or_else(|| format!("Playlist {}", playlist_id));
        let current_track_id = self.session.playback_state.current_track_id;
        let track_ids = self.activate_playlist_queue(playlist_id, playlist_name)?;

        if track_ids.is_empty() {
            self.session.playback_state.current_track_id = None;
            self.session.playback_state.position_secs = 0;
            self.persist_playback_state()?;
        } else if let Some(current_track_id) = current_track_id
            && !track_ids.contains(&current_track_id)
        {
            self.play_track(track_ids[0])?;
        }

        Ok(())
    }

    pub(super) fn persist_queue(&mut self) -> Result<()> {
        self.storage.replace_queue(&self.session.queue)?;
        self.session.queue_version = self.session.queue_version.saturating_add(1);
        Ok(())
    }

    pub(super) fn clear_active_playlist_context(&mut self) {
        self.session.active_playlist_id = None;
        self.session.active_playlist_name = None;
    }
}
