use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

use super::state::{FocusPanel, InputMode, UiState};

impl UiState {
    pub fn handle_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        if key.kind != event::KeyEventKind::Press {
            return Ok(false);
        }

        if self.mode == InputMode::Search {
            return self.handle_search_input(app, key);
        }

        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('h') => self.focus = FocusPanel::Sidebar,
            KeyCode::Char('l') => self.focus = FocusPanel::Library,
            KeyCode::Char('j') | KeyCode::Down => self.move_selection(app, 1),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection(app, -1),
            KeyCode::Enter => self.play_selected(app)?,
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
            KeyCode::Char('/') => {
                self.mode = InputMode::Search;
                self.search_input.clear();
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
            _ => {}
        }

        Ok(false)
    }

    fn handle_search_input(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = InputMode::Normal;
                self.search_input.clear();
                self.selected = 0;
                self.status = String::from("Back to normal mode");
            }
            KeyCode::Enter => {
                self.play_selected(app)?;
                self.mode = InputMode::Normal;
                self.status = String::from("Play from search results");
            }
            KeyCode::Backspace => {
                self.search_input.pop();
                self.selected = 0;
            }
            KeyCode::Down | KeyCode::Char('j') => self.move_selection(app, 1),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection(app, -1),
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.search_input.push(c);
                    self.selected = 0;
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
}
