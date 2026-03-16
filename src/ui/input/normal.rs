use super::*;

impl UiState {
    pub(super) fn handle_normal_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::BackTab | KeyCode::Tab => self.focus_right(),
            KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => {
                if matches!(self.focus, FocusPanel::Queue)
                    && let Some(next) = app.move_queue_index(self.queue_selected, -1)?
                {
                    self.queue_selected = next;
                    self.status = format!("Moved queue item to #{}", next + 1);
                }
            }
            KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => {
                if matches!(self.focus, FocusPanel::Queue)
                    && let Some(next) = app.move_queue_index(self.queue_selected, 1)?
                {
                    self.queue_selected = next;
                    self.status = format!("Moved queue item to #{}", next + 1);
                }
            }
            KeyCode::Down => self.move_selection(app, 1),
            KeyCode::Up => self.move_selection(app, -1),
            KeyCode::Enter => match self.focus {
                FocusPanel::Queue => self.play_selected_queue(app)?,
                _ => self.play_selected(app)?,
            },
            KeyCode::Char(' ') => {
                let paused = app.toggle_play_pause()?;
                self.status = if paused {
                    String::from("Paused")
                } else {
                    String::from("Playing")
                };
            }
            KeyCode::Char('n') => {
                app.next_track()?;
                self.status = String::from("Next track");
            }
            KeyCode::Char('p') => {
                app.prev_track()?;
                self.status = String::from("Previous track");
            }
            KeyCode::Left => {
                let delta = if key.modifiers.contains(KeyModifiers::SHIFT) {
                    -10
                } else {
                    -5
                };
                app.seek(delta)?;
                self.status = format!("Seek {delta}s");
            }
            KeyCode::Right => {
                let delta = if key.modifiers.contains(KeyModifiers::SHIFT) {
                    10
                } else {
                    5
                };
                app.seek(delta)?;
                self.status = format!("Seek +{delta}s");
            }
            KeyCode::Char('s') => {
                self.enter_search_mode();
                self.status = String::from("Search mode");
            }
            KeyCode::Char('r') => {
                if app.begin_scan() {
                    self.status = String::from("Rescan started...");
                } else {
                    self.status = String::from("Rescan already running");
                }
            }
            KeyCode::Char('f') => {
                if let Some(track_id) = self.selected_track_id(app) {
                    app.toggle_favorite(track_id)?;
                    self.status = format!("Favorite toggled #{}", track_id);
                }
            }
            KeyCode::Char('a') => {
                if let Some(track_id) = self.selected_track_id(app)
                    && app.add_to_queue(track_id)?
                {
                    self.status = format!("Queued #{}", track_id);
                }
            }
            KeyCode::Char('e') => {
                let mode = app.toggle_repeat()?;
                self.status = format!("Repeat: {mode}");
            }
            KeyCode::Char('u') => {
                let on = app.toggle_shuffle()?;
                self.status = if on {
                    String::from("Shuffle: On")
                } else {
                    String::from("Shuffle: Off")
                };
            }
            KeyCode::Char(']') => {
                let volume = app.adjust_volume(0.05);
                self.status = format!("Volume: {volume}%");
            }
            KeyCode::Char('[') => {
                let volume = app.adjust_volume(-0.05);
                self.status = format!("Volume: {volume}%");
            }
            KeyCode::Char('x') => {
                if matches!(self.focus, FocusPanel::Queue)
                    && let Some(track_id) = app.remove_queue_index(self.queue_selected)?
                {
                    self.status = format!("Removed from queue #{}", track_id);
                    let queue_len = app.queue_len();
                    self.queue_selected = self.queue_selected.min(queue_len.saturating_sub(1));
                }
            }
            KeyCode::Char('m') => {
                if let Some(track_id) = self.selected_track_id(app)
                    && let Some(track) = app.track_by_id(track_id)
                {
                    let current_title = track.title.clone();
                    self.enter_rename_mode(track_id, &current_title);
                    self.status = String::from("Rename mode - Enter to confirm, Esc to cancel");
                }
            }
            KeyCode::Char('M') => {
                if let Some(track_id) = self.selected_track_id(app)
                    && let Some(track) = app.track_by_id(track_id)
                {
                    let current_artist = track.artist.clone().unwrap_or_default();
                    self.enter_rename_artist_mode(track_id, &current_artist);
                    self.status = String::from("Rename artist - Enter to confirm, Esc to cancel");
                }
            }
            KeyCode::Char('t') => {
                if let Some(track_id) = self.selected_track_id(app)
                    && let Some(track) = app.track_by_id(track_id)
                {
                    let title = track.title.clone();
                    let artist = track.artist.clone().unwrap_or_default();
                    let album = track.album.clone().unwrap_or_default();
                    self.enter_edit_tag_mode(track_id, &title, &artist, &album);
                    self.status =
                        String::from("Edit tags - Tab: next field, Enter: save, Esc: cancel");
                }
            }
            _ => {}
        }

        Ok(false)
    }
}
