use super::*;

impl UiState {
    fn handle_normal_navigation(&mut self, app: &mut App, key: KeyEvent) -> Result<Option<bool>> {
        match key.code {
            KeyCode::Char('q') => Ok(Some(true)),
            KeyCode::Down => {
                self.move_selection(app, 1);
                Ok(Some(false))
            }
            KeyCode::Up => {
                self.move_selection(app, -1);
                Ok(Some(false))
            }
            KeyCode::Enter => {
                self.play_selected(app)?;
                Ok(Some(false))
            }
            _ => Ok(None),
        }
    }

    fn handle_normal_playback_controls(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
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
            KeyCode::Left | KeyCode::Right => {
                let step = if key.modifiers.contains(KeyModifiers::SHIFT) {
                    10
                } else {
                    5
                };
                let delta = if matches!(key.code, KeyCode::Left) {
                    -step
                } else {
                    step
                };
                app.seek(delta)?;
                self.status = if delta > 0 {
                    format!("Seek +{delta}s")
                } else {
                    format!("Seek {delta}s")
                };
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
            KeyCode::Char(']') | KeyCode::Char('[') => {
                let delta = if matches!(key.code, KeyCode::Char(']')) {
                    0.05
                } else {
                    -0.05
                };
                let volume = app.adjust_volume(delta);
                self.status = format!("Volume: {volume}%");
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn enter_unified_tag_mode_for_selected_track(&mut self, app: &App) {
        if let Some(track_id) = self.selected_track_id(app)
            && let Some(track) = app.track_by_id(track_id)
        {
            let title = track.title.clone();
            let artist = track.artist.clone().unwrap_or_default();
            let album = track.album.clone().unwrap_or_default();
            self.enter_edit_tag_mode(track_id, &title, &artist, &album);
            self.status = String::from(
                "Edit metadata - Tab: next field, Enter: save (artist/title rename filename), Esc: cancel",
            );
        }
    }

    fn quick_add_selected_track_to_playlist(&mut self, app: &mut App) -> Result<()> {
        let track_id = self.resolve_track_for_playlist_add(app);
        let Some(track_id) = track_id else {
            self.status = String::from("No track selected to add");
            return Ok(());
        };

        self.enter_playlist_modal();
        self.playlist_add_track_id = Some(track_id);
        let track_label = self.track_label_by_id(app, track_id);
        self.status = format!("Select playlist for {}", track_label);
        Ok(())
    }

    fn handle_normal_library_actions(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('s') => {
                self.enter_search_mode();
                self.status = String::from("Search mode");
            }
            KeyCode::Char('l') => {
                self.enter_playlist_modal();
                self.status = String::from("Playlist modal");
            }
            KeyCode::Char('r') => {
                self.status = if app.begin_scan() {
                    String::from("Rescan started...")
                } else {
                    String::from("Rescan already running")
                };
            }
            KeyCode::Char('f') => {
                if let Some(track_id) = self.selected_track_id(app) {
                    app.toggle_favorite(track_id)?;
                    self.status = format!("Favorite toggled #{}", track_id);
                }
            }
            KeyCode::Char('a') => {
                self.quick_add_selected_track_to_playlist(app)?;
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.enter_unified_tag_mode_for_selected_track(app);
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    pub(super) fn handle_normal_key(&mut self, app: &mut App, key: KeyEvent) -> Result<bool> {
        if let Some(should_quit) = self.handle_normal_navigation(app, key)? {
            return Ok(should_quit);
        }

        if self.handle_normal_playback_controls(app, key)? {
            return Ok(false);
        }

        let _ = self.handle_normal_library_actions(app, key)?;

        Ok(false)
    }
}
