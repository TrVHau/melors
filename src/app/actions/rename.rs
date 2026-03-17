use super::*;

impl App {
    pub fn rename_track(&mut self, track_id: i64, new_title: &str) -> Result<()> {
        let (old_path, new_path) = {
            let track = match self.track_by_id(track_id) {
                Some(t) => t,
                None => return Ok(()),
            };
            let old_path = track.path.clone();
            let ext = old_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let new_filename = if ext.is_empty() {
                new_title.to_string()
            } else {
                format!("{}.{}", new_title, ext)
            };
            let new_path = old_path
                .parent()
                .unwrap_or(Path::new("."))
                .join(&new_filename);
            (old_path, new_path)
        };
        std::fs::rename(&old_path, &new_path)?;
        self.storage
            .rename_track(track_id, new_title, &new_path.to_string_lossy())?;
        self.reload_session_state()
    }

    pub fn rename_artist(&mut self, track_id: i64, new_artist: &str) -> Result<()> {
        if self.track_by_id(track_id).is_none() {
            return Ok(());
        }
        self.storage.rename_artist(track_id, new_artist)?;
        self.reload_session_state()
    }

    pub fn write_track_tags(
        &mut self,
        track_id: i64,
        title: &str,
        artist: &str,
        album: &str,
    ) -> Result<()> {
        let path = match self.track_by_id(track_id) {
            Some(t) => t.path.clone(),
            None => return Ok(()),
        };
        crate::services::metadata::write_tag(
            &path,
            title,
            if artist.is_empty() {
                None
            } else {
                Some(artist)
            },
            if album.is_empty() { None } else { Some(album) },
        )?;
        self.storage
            .rename_track(track_id, title, &path.to_string_lossy())?;
        self.storage.rename_artist(track_id, artist)?;
        self.storage.rename_album(track_id, album)?;
        self.reload_session_state()
    }
}
