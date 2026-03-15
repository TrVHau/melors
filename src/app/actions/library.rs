use super::*;

impl App {
    pub fn toggle_favorite(&mut self, track_id: i64) -> Result<()> {
        self.storage.toggle_favorite(track_id)?;
        self.reload_session_state()?;
        Ok(())
    }

    pub fn current_track(&self) -> Option<&Track> {
        self.session
            .playback_state
            .current_track_id
            .and_then(|id| self.track_by_id(id))
    }

    pub fn tracks_version(&self) -> u64 {
        self.session.tracks_version
    }

    pub fn tracks(&self) -> &[Track] {
        &self.session.tracks
    }

    pub fn playback_state(&self) -> &PlaybackState {
        &self.session.playback_state
    }

    pub fn track_by_id(&self, track_id: i64) -> Option<&Track> {
        self.session
            .track_index_by_id
            .get(&track_id)
            .and_then(|idx| self.session.tracks.get(*idx))
    }

    pub(super) fn build_track_index(tracks: &[Track]) -> HashMap<i64, usize> {
        tracks
            .iter()
            .enumerate()
            .map(|(idx, track)| (track.id, idx))
            .collect()
    }
}
