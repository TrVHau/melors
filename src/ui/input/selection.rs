use super::*;

impl UiState {
    pub(super) fn play_selected(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = self.selected_track_id(app) {
            app.play_track(track_id)?;
            self.status = format!("Playing #{}", track_id);
        }
        Ok(())
    }
}
