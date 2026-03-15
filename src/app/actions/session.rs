use super::*;

impl App {
    pub(super) fn reload_session_state(&mut self) -> Result<()> {
        self.session.tracks = self.storage.load_tracks()?;
        self.session.track_index_by_id = Self::build_track_index(&self.session.tracks);
        self.session.tracks_version = self.session.tracks_version.saturating_add(1);
        self.session.queue = self.storage.load_queue()?;
        self.session.queue_version = self.session.queue_version.saturating_add(1);
        self.normalize_queue()?;

        let current_track_missing = self
            .session
            .playback_state
            .current_track_id
            .is_some_and(|track_id| !self.session.tracks.iter().any(|track| track.id == track_id));

        if current_track_missing {
            self.player.stop();
            self.session.playback_state.current_track_id = None;
            self.session.playback_state.position_secs = 0;
            self.persist_playback_state()?;
        }

        Ok(())
    }
}
