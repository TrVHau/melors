use super::*;
use crate::ui::state::PlaylistModalMode;

impl UiState {
    fn accepts_plain_text(key: KeyEvent) -> bool {
        !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT)
    }

    fn create_playlist_with_name(&mut self, app: &mut App, name: &str) -> Result<Option<i64>> {
        let msg = app.create_playlist_action(name).status_message();
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

    fn default_playlist_name(&self, app: &App) -> Result<String> {
        Ok(format!(
            "Playlist {}",
            app.list_playlists_action()?.len() + 1
        ))
    }

    pub(super) fn resolve_track_for_playlist_add(&mut self, app: &mut App) -> Option<i64> {
        let selected_library_track = self.selected_track_id(app);
        let current_playing_track = app.playback_state().current_track_id;
        selected_library_track.or(current_playing_track)
    }

    fn handle_create_playlist_mode(&mut self, app: &mut App, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                self.playlist_rename_input.clear();
                self.status = String::from("Create playlist cancelled");
            }
            KeyCode::Enter => {
                let requested_name = self.playlist_rename_input.trim().to_string();
                let Some(playlist_id) = self.create_playlist_with_name(app, &requested_name)?
                else {
                    return Ok(());
                };

                if let Some(track_id) = self.playlist_add_track_id {
                    let msg = app
                        .add_playlist_item_action(playlist_id, track_id)
                        .status_message();
                    self.status = format!("{} (#{} <- #{})", msg.text, playlist_id, track_id);
                    self.exit_playlist_modal();
                } else {
                    self.playlist_modal_mode = PlaylistModalMode::BrowseItems;
                    self.playlist_item_selected = 0;
                    self.playlist_rename_input.clear();
                }
            }
            KeyCode::Backspace => {
                self.playlist_rename_input.pop();
            }
            KeyCode::Char(c) if Self::accepts_plain_text(key) => {
                self.playlist_rename_input.push(c);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_rename_playlist_mode(&mut self, app: &mut App, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                self.playlist_rename_input.clear();
                self.status = String::from("Rename cancelled");
            }
            KeyCode::Enter => {
                let Some(playlist_id) = self.selected_playlist_id(app)? else {
                    self.status = String::from("No playlist selected");
                    self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                    self.playlist_rename_input.clear();
                    return Ok(());
                };

                let msg = app
                    .rename_playlist_action(playlist_id, &self.playlist_rename_input)
                    .status_message();
                self.status = msg.text;
                self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                self.playlist_rename_input.clear();
            }
            KeyCode::Backspace => {
                self.playlist_rename_input.pop();
            }
            KeyCode::Char(c) if Self::accepts_plain_text(key) => {
                self.playlist_rename_input.push(c);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_delete_playlist_confirmation(&mut self, app: &mut App, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                self.status = String::from("Delete cancelled");
            }
            KeyCode::Enter | KeyCode::Char('x') => {
                let Some(playlist_id) = self.selected_playlist_id(app)? else {
                    self.status = String::from("Select a playlist to delete");
                    self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                    return Ok(());
                };

                let msg = app.delete_playlist_action(playlist_id).status_message();
                self.status = msg.text;

                let playlists_len = app.list_playlists_action()?.len();
                self.playlist_selected = self.playlist_selected.min(playlists_len);
                self.playlist_item_selected = 0;
                self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_playlist_browse_enter(&mut self, app: &mut App) -> Result<()> {
        if let Some(track_id) = self.playlist_add_track_id {
            let playlist_id = if let Some(playlist_id) = self.selected_playlist_id(app)? {
                playlist_id
            } else {
                self.playlist_rename_input = self.default_playlist_name(app)?;
                self.playlist_modal_mode = PlaylistModalMode::CreatePlaylist;
                self.status = String::from("Name the new playlist");
                return Ok(());
            };

            let msg = app
                .add_playlist_item_action(playlist_id, track_id)
                .status_message();
            self.status = format!("{} (#{} <- #{})", msg.text, playlist_id, track_id);
            self.exit_playlist_modal();
            return Ok(());
        }

        if let Some(playlist_id) = self.selected_playlist_id(app)? {
            self.playlist_modal_mode = PlaylistModalMode::BrowseItems;
            self.playlist_item_selected = 0;
            self.status = format!("Opened playlist #{}", playlist_id);
        } else {
            self.playlist_rename_input = self.default_playlist_name(app)?;
            self.playlist_modal_mode = PlaylistModalMode::CreatePlaylist;
            self.status = String::from("Name the new playlist");
        }

        Ok(())
    }

    fn remove_selected_playlist_item(&mut self, app: &mut App) -> Result<()> {
        let Some(playlist_id) = self.selected_playlist_id(app)? else {
            self.status = String::from("No playlist selected");
            return Ok(());
        };

        let items = app.list_playlist_items_action(playlist_id)?;
        if items.is_empty() {
            self.status = String::from("Playlist is empty");
            return Ok(());
        }

        let idx = self.playlist_item_selected.min(items.len() - 1);
        let order_index = items[idx].order_index;
        let msg = app
            .remove_playlist_item_action(playlist_id, order_index)
            .status_message();
        self.status = msg.text;

        let remaining = app.list_playlist_items_action(playlist_id)?.len();
        self.playlist_item_selected = if remaining == 0 {
            0
        } else {
            self.playlist_item_selected.min(remaining - 1)
        };

        Ok(())
    }

    fn start_playlist_rename(&mut self, app: &mut App) -> Result<()> {
        let Some(playlist_id) = self.selected_playlist_id(app)? else {
            self.status = String::from("Select a playlist to rename");
            return Ok(());
        };

        let playlists = app.list_playlists_action()?;
        if let Some(playlist) = playlists.iter().find(|p| p.id == playlist_id) {
            self.playlist_rename_input = playlist.name.clone();
            self.playlist_modal_mode = PlaylistModalMode::RenamePlaylist;
            self.status = format!("Rename playlist #{}", playlist_id);
        }

        Ok(())
    }

    fn request_delete_selected_playlist(&mut self, app: &mut App) -> Result<()> {
        if self.playlist_add_track_id.is_some() {
            return Ok(());
        }

        let Some(playlist_id) = self.selected_playlist_id(app)? else {
            self.status = String::from("Select a playlist to delete");
            return Ok(());
        };

        let playlist_name = app
            .list_playlists_action()?
            .into_iter()
            .find(|playlist| playlist.id == playlist_id)
            .map(|playlist| playlist.name)
            .unwrap_or_else(|| format!("#{}", playlist_id));
        self.playlist_modal_mode = PlaylistModalMode::ConfirmDeletePlaylist;
        self.status = format!("Delete playlist {}?", playlist_name);
        Ok(())
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
                if Self::accepts_plain_text(key) {
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
                if Self::accepts_plain_text(key) {
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
        if self.playlist_modal_mode == PlaylistModalMode::CreatePlaylist {
            self.handle_create_playlist_mode(app, key)?;
            return Ok(false);
        }

        if self.playlist_modal_mode == PlaylistModalMode::RenamePlaylist {
            self.handle_rename_playlist_mode(app, key)?;
            return Ok(false);
        }

        if self.playlist_modal_mode == PlaylistModalMode::ConfirmDeletePlaylist {
            self.handle_delete_playlist_confirmation(app, key)?;
            return Ok(false);
        }

        match key.code {
            KeyCode::Esc => match self.playlist_modal_mode {
                PlaylistModalMode::BrowseItems => {
                    self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
                    self.playlist_item_selected = 0;
                    self.status = String::from("Back to playlists");
                }
                PlaylistModalMode::BrowsePlaylists => {
                    self.exit_playlist_modal();
                    self.status = String::from("Closed playlist modal");
                }
                PlaylistModalMode::CreatePlaylist => unreachable!(),
                PlaylistModalMode::RenamePlaylist => unreachable!(),
                PlaylistModalMode::ConfirmDeletePlaylist => unreachable!(),
            },
            KeyCode::Char('l') => {
                self.exit_playlist_modal();
                self.status = String::from("Closed playlist modal");
            }
            KeyCode::Down => {
                if self.playlist_modal_mode == PlaylistModalMode::BrowsePlaylists {
                    self.move_playlist_selection(app, 1)?;
                } else if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                    self.move_playlist_item_selection(app, 1)?;
                }
            }
            KeyCode::Up => {
                if self.playlist_modal_mode == PlaylistModalMode::BrowsePlaylists {
                    self.move_playlist_selection(app, -1)?;
                } else if self.playlist_modal_mode == PlaylistModalMode::BrowseItems {
                    self.move_playlist_item_selection(app, -1)?;
                }
            }
            KeyCode::Enter => match self.playlist_modal_mode {
                PlaylistModalMode::BrowsePlaylists => {
                    self.handle_playlist_browse_enter(app)?;
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
                PlaylistModalMode::CreatePlaylist => unreachable!(),
                PlaylistModalMode::RenamePlaylist => unreachable!(),
                PlaylistModalMode::ConfirmDeletePlaylist => unreachable!(),
            },
            KeyCode::Char('d') => {
                if self.playlist_modal_mode != PlaylistModalMode::BrowseItems {
                    return Ok(false);
                }
                self.remove_selected_playlist_item(app)?;
            }
            KeyCode::Char('r') => {
                if self.playlist_modal_mode != PlaylistModalMode::BrowsePlaylists {
                    return Ok(false);
                }
                self.start_playlist_rename(app)?;
            }
            KeyCode::Char('x') => {
                if self.playlist_modal_mode != PlaylistModalMode::BrowsePlaylists {
                    return Ok(false);
                }
                self.request_delete_selected_playlist(app)?;
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
