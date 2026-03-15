use super::*;

impl UiState {
    pub(super) fn play_selected(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = self.selected_track_id(app) {
            app.play_track(track_id)?;
            self.status = format!("Playing #{}", track_id);
        }
        Ok(())
    }

    pub(super) fn play_selected_queue(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = app.play_queue_index(self.queue_selected)? {
            self.status = format!("Playing from queue #{}", track_id);
        }
        Ok(())
    }
}
