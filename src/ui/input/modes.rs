use super::*;

impl UiState {
    pub(super) fn handle_edit_tag_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.exit_edit_tag_mode();
                self.status = String::from("Edit tags cancelled");
            }
            KeyCode::Tab | KeyCode::BackTab => {
                self.edit_tag_field = (self.edit_tag_field + 1) % 3;
            }
            KeyCode::Enter => {
                if let Some(track_id) = self.edit_tag_track_id {
                    let title = self.edit_tag_inputs[0].trim().to_string();
                    if !title.is_empty() {
                        let artist = self.edit_tag_inputs[1].trim().to_string();
                        let album = self.edit_tag_inputs[2].trim().to_string();
                        match app.write_track_tags(track_id, &title, &artist, &album) {
                            Ok(()) => self.status = format!("Tags saved for #{}", track_id),
                            Err(e) => self.status = format!("Save failed: {}", e),
                        }
                    }
                }
                self.exit_edit_tag_mode();
            }
            KeyCode::Backspace => {
                self.edit_tag_inputs[self.edit_tag_field].pop();
            }
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.edit_tag_inputs[self.edit_tag_field].push(c);
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(super) fn handle_rename_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.exit_rename_mode();
                self.status = String::from("Rename cancelled");
            }
            KeyCode::Enter => {
                let new_value = self.rename_input.trim().to_string();
                if let Some(track_id) = self.rename_track_id
                    && !new_value.is_empty()
                {
                    let result = match self.rename_kind {
                        RenameKind::Title => app.rename_track(track_id, &new_value),
                        RenameKind::Artist => app.rename_artist(track_id, &new_value),
                    };
                    let label = match self.rename_kind {
                        RenameKind::Title => "title",
                        RenameKind::Artist => "artist",
                    };
                    match result {
                        Ok(()) => self.status = format!("Renamed {} #{} -> {}", label, track_id, new_value),
                        Err(e) => self.status = format!("Rename failed: {}", e),
                    }
                }
                self.exit_rename_mode();
            }
            KeyCode::Backspace => {
                self.rename_input.pop();
            }
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.rename_input.push(c);
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(super) fn handle_search_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.exit_search_mode();
                self.status = String::from("Back to normal mode");
            }
            KeyCode::Enter => {
                self.play_selected(app)?;
                self.exit_search_mode();
                self.status = String::from("Play from search results");
            }
            KeyCode::Backspace => {
                self.search_input.pop();
                self.library_selected = 0;
            }
            KeyCode::Down => self.move_selection(app, 1),
            KeyCode::Up => self.move_selection(app, -1),
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.search_input.push(c);
                    self.library_selected = 0;
                }
            }
            _ => {}
        }
        Ok(false)
    }
}
