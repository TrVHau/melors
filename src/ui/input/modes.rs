use super::*;
use crate::ui::state::PlaylistModalMode;

impl UiState {
    fn create_next_playlist(&mut self, app: &mut App) -> Result<Option<i64>> {
        let name = format!("Playlist {}", app.list_playlists_action()?.len() + 1);
        let msg = app.create_playlist_action(&name).status_message();
        self.status = msg.text;
        if msg.code != "playlist.created" {
            return Ok(None);
        }

        let playlists = app.list_playlists_action()?;
        if playlists.is_empty() {
            self.status = String::from("Create playlist failed");
            return Ok(None);
        }
        self.playlist_selected = playlists.len() - 1;
        Ok(playlists.last().map(|p| p.id))
    }

    pub(super) fn resolve_track_for_playlist_add(&mut self, app: &mut App) -> Option<i64> {
        let selected_library_track = self.selected_track_id(app);
        let current_playing_track = app.playback_state().current_track_id;
        selected_library_track.or(current_playing_track)
    }

    pub(super) fn ensure_playlist_for_add(&mut self, app: &mut App) -> Result<Option<i64>> {
        if let Some(id) = self.selected_playlist_id(app)? {
            return Ok(Some(id));
        }
        self.create_next_playlist(app)
    }

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
                    if let Err(msg) = Self::validate_tag_edit_fields(&title) {
                        self.status = format!("Save failed: {}", msg);
                        return Ok(false);
                    }

                    let artist = self.edit_tag_inputs[1].trim().to_string();
                    let album = self.edit_tag_inputs[2].trim().to_string();
                    match app.write_track_tags(track_id, &title, &artist, &album) {
                        Ok(()) => {
                            self.status = format!("Tags saved for #{}", track_id);
                            self.exit_edit_tag_mode();
                        }
                        Err(e) => {
                            self.status = format!("Save failed: {} (fix fields and retry)", e);
                            return Ok(false);
                        }
                    }
                }
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
                self.search_warning = Self::search_warning_for_query(&self.search_input);
            }
            KeyCode::Down => self.move_selection(app, 1),
            KeyCode::Up => self.move_selection(app, -1),
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT)
                {
                    self.search_input.push(c);
                    self.library_selected = 0;
                    self.search_warning = Self::search_warning_for_query(&self.search_input);
                }
            }
            _ => {}
        }
        Ok(false)
    }

    pub(super) fn handle_playlist_modal_input(
        &mut self,
        app: &mut App,
        key: KeyEvent,
    ) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                    self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                    self.playlist_item_selected = 0;
                    self.status = String::from("Back to playlists");
                } else {
                    self.exit_playlist_modal();
                    self.status = String::from("Closed playlist modal");
                }
            }
            KeyCode::Char('l') => {
                self.exit_playlist_modal();
                self.status = String::from("Closed playlist modal");
            }
            KeyCode::Down => {
                if self.playlist_modal_mode == PlaylistModalMode::BrowsePlaylists {
                    self.move_playlist_selection(app, 1)?;
                } else {
                    self.move_playlist_item_selection(app, 1)?;
                }
            }
            KeyCode::Up => {
                if self.playlist_modal_mode == PlaylistModalMode::BrowsePlaylists {
                    self.move_playlist_selection(app, -1)?;
                } else {
                    self.move_playlist_item_selection(app, -1)?;
                }
            }
            KeyCode::Enter => {
                match self.playlist_modal_mode {
                    PlaylistModalMode::BrowsePlaylists => {
                        if let Some(playlist_id) = self.selected_playlist_id(app)? {
                            self.playlist_modal_mode = PlaylistModalMode::BrowseItems;
                            self.playlist_item_selected = 0;
                            self.status = format!("Opened playlist #{}", playlist_id);
                        } else if self.create_next_playlist(app)?.is_some() {
                            self.playlist_modal_mode = PlaylistModalMode::BrowseItems;
                            self.playlist_item_selected = 0;
                        }
                    }
                    PlaylistModalMode::BrowseItems => {
                        let Some(playlist_id) = self.selected_playlist_id(app)? else {
                            self.status = String::from("No playlist selected");
                            return Ok(false);
                        };
                        let result = app.play_from_playlist_action(
                            playlist_id,
                            Some(self.selected_playlist_item_index()),
                        );
                        let msg = result.status_message();
                        self.status = msg.text;
                    }
                }
            }
            KeyCode::Char('a') => {
                let Some(playlist_id) = self.ensure_playlist_for_add(app)? else {
                    return Ok(false);
                };

                let Some(track_id) = self.resolve_track_for_playlist_add(app) else {
                    self.status = String::from(
                        "No track to add. Select a Library track (or play one) then press [a]",
                    );
                    return Ok(false);
                };

                let msg = app
                    .add_playlist_item_action(playlist_id, track_id)
                    .status_message();
                self.playlist_modal_mode = PlaylistModalMode::BrowseItems;
                self.status = format!("{} (#{} <- #{})", msg.text, playlist_id, track_id);
            }
            _ => {}
        }
        Ok(false)
    }

    pub(crate) fn search_warning_for_query(raw_query: &str) -> Option<String> {
        for token in raw_query.split_whitespace() {
            let Some((key, value)) = token.split_once(':') else {
                continue;
            };
            if value.trim().is_empty() {
                continue;
            }
            let key = key.to_ascii_lowercase();
            match key.as_str() {
                "artist" | "album" => {}
                "fav" | "favorite" => {
                    let v = value.to_ascii_lowercase();
                    let valid = matches!(v.as_str(), "1" | "0" | "true" | "false" | "yes" | "no");
                    if !valid {
                        return Some(format!(
                            "Invalid favorite filter `{}`; using fallback search",
                            token
                        ));
                    }
                }
                _ => return Some(format!("Unknown filter `{}`; using fallback search", key)),
            }
        }
        None
    }

    pub(crate) fn validate_tag_edit_fields(title: &str) -> std::result::Result<(), &'static str> {
        if title.trim().is_empty() {
            return Err("title cannot be empty");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_warning_detects_invalid_favorite_token() {
        let warning = UiState::search_warning_for_query("fav:maybe test");
        assert!(warning.is_some());
    }

    #[test]
    fn search_warning_detects_unknown_filter_token() {
        let warning = UiState::search_warning_for_query("genre:rock");
        assert!(warning.is_some());
    }

    #[test]
    fn search_warning_none_for_supported_filters() {
        let warning = UiState::search_warning_for_query("artist:radiohead album:kid fav:true");
        assert!(warning.is_none());
    }

    #[test]
    fn tag_validation_rejects_empty_title() {
        assert!(UiState::validate_tag_edit_fields("").is_err());
    }
}
