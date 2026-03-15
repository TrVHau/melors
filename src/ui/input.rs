use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

use super::state::{FocusPanel, InputMode, RenameKind, UiState, VisualizerMode};

impl UiState {
    pub fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        if key.kind != event::KeyEventKind::Press {
            return Ok(false);
        }

        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('1') => {
                    self.set_visualizer_mode(VisualizerMode::Cava);
                    self.status = String::from("Visualizer: Cava");
                    return Ok(false);
                }
                KeyCode::Char('2') => {
                    self.set_visualizer_mode(VisualizerMode::Clock);
                    self.status = String::from("Visualizer: Clock");
                    return Ok(false);
                }
                KeyCode::Char('3') => {
                    self.set_visualizer_mode(VisualizerMode::CMatrix);
                    self.status = String::from("Visualizer: CMatrix");
                    return Ok(false);
                }
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    let theme = self.cycle_theme();
                    self.status = format!("Theme: {}", theme);
                    return Ok(false);
                }
                _ => {}
            }
        }

        if self.mode == InputMode::Search {
            return self.handle_search_input(app, key);
        }

        if self.mode == InputMode::Rename {
            return self.handle_rename_input(app, key);
        }

        if self.mode == InputMode::EditTag {
            return self.handle_edit_tag_input(app, key);
        }

        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::BackTab => self.focus_right(),
            KeyCode::Tab => self.focus_right(),
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
                app.scan_now()?;
                self.status = String::from("Rescan complete");
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
                    self.status = String::from("Rename mode — Enter to confirm, Esc to cancel");
                }
            }
            KeyCode::Char('M') => {
                if let Some(track_id) = self.selected_track_id(app)
                    && let Some(track) = app.track_by_id(track_id)
                {
                    let current_artist = track.artist.clone().unwrap_or_default();
                    self.enter_rename_artist_mode(track_id, &current_artist);
                    self.status = String::from("Rename artist — Enter to confirm, Esc to cancel");
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
                    self.status = String::from("Edit tags — Tab: next field, Enter: save, Esc: cancel");
                }
            }
            _ => {}
        }

        Ok(false)
    }

    fn handle_edit_tag_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.exit_edit_tag_mode();
                self.status = String::from("Edit tags cancelled");
            }
            KeyCode::Tab => {
                self.edit_tag_field = (self.edit_tag_field + 1) % 3;
            }
            KeyCode::BackTab => {
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

    fn handle_rename_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
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
                        Ok(()) => {
                            self.status =
                                format!("Renamed {} #{} → {}", label, track_id, new_value)
                        }
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

    fn handle_search_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
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

    fn play_selected(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = self.selected_track_id(app) {
            app.play_track(track_id)?;
            self.status = format!("Playing #{}", track_id);
        }
        Ok(())
    }

    fn play_selected_queue(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = app.play_queue_index(self.queue_selected)? {
            self.status = format!("Playing from queue #{}", track_id);
        }
        Ok(())
    }
}
