use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStatusLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionStatusMessage {
    pub level: ActionStatusLevel,
    pub code: &'static str,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaylistActionResult {
    PlaylistCreated {
        playlist_id: i64,
        name: String,
    },
    PlaylistDeleted {
        playlist_id: i64,
    },
    PlaylistRenamed {
        playlist_id: i64,
        name: String,
    },
    PlaylistItemAdded {
        playlist_id: i64,
        track_id: i64,
    },
    PlaylistItemRemoved {
        playlist_id: i64,
        order_index: i64,
    },
    PlaybackStarted {
        playlist_id: i64,
        track_id: i64,
    },
    NoPlayableItem {
        playlist_id: i64,
    },
    ValidationError {
        context: &'static str,
        message: String,
    },
    NotFoundError {
        context: &'static str,
        message: String,
    },
    PersistenceError {
        context: &'static str,
        message: String,
    },
}

impl PlaylistActionResult {
    pub fn status_message(&self) -> ActionStatusMessage {
        match self {
            Self::PlaylistCreated { name, .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.created",
                text: format!("Playlist created: {name}"),
            },
            Self::PlaylistDeleted { .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.deleted",
                text: String::from("Playlist deleted"),
            },
            Self::PlaylistRenamed { name, .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.renamed",
                text: format!("Playlist renamed: {name}"),
            },
            Self::PlaylistItemAdded { .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.item_added",
                text: String::from("Track added to playlist"),
            },
            Self::PlaylistItemRemoved { .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.item_removed",
                text: String::from("Track removed from playlist"),
            },
            Self::PlaybackStarted { .. } => ActionStatusMessage {
                level: ActionStatusLevel::Info,
                code: "playlist.playback_started",
                text: String::from("Playback started from playlist"),
            },
            Self::NoPlayableItem { .. } => ActionStatusMessage {
                level: ActionStatusLevel::Warning,
                code: "playlist.no_playable_item",
                text: String::from("No playable item found in playlist"),
            },
            Self::ValidationError { message, .. } => ActionStatusMessage {
                level: ActionStatusLevel::Error,
                code: "playlist.validation_error",
                text: message.clone(),
            },
            Self::NotFoundError { message, .. } => ActionStatusMessage {
                level: ActionStatusLevel::Error,
                code: "playlist.not_found",
                text: message.clone(),
            },
            Self::PersistenceError { message, .. } => ActionStatusMessage {
                level: ActionStatusLevel::Error,
                code: "playlist.persistence_error",
                text: message.clone(),
            },
        }
    }
}

impl App {
    pub fn create_playlist_action(&self, name: &str) -> PlaylistActionResult {
        match self.storage.create_playlist(name) {
            Ok(playlist_id) => PlaylistActionResult::PlaylistCreated {
                playlist_id,
                name: name.trim().to_string(),
            },
            Err(err) => classify_storage_error("create_playlist", err),
        }
    }

    pub fn delete_playlist_action(&mut self, playlist_id: i64) -> PlaylistActionResult {
        match self.storage.delete_playlist(playlist_id) {
            Ok(()) => {
                if self.session.active_playlist_id == Some(playlist_id) {
                    self.clear_active_playlist_context();
                    if let Err(err) = self.rebuild_queue() {
                        return classify_storage_error("delete_playlist", err);
                    }
                }
                PlaylistActionResult::PlaylistDeleted { playlist_id }
            }
            Err(err) => classify_storage_error("delete_playlist", err),
        }
    }

    pub fn rename_playlist_action(&mut self, playlist_id: i64, name: &str) -> PlaylistActionResult {
        match self.storage.rename_playlist(playlist_id, name) {
            Ok(()) => {
                if self.session.active_playlist_id == Some(playlist_id) {
                    self.session.active_playlist_name = Some(name.trim().to_string());
                }
                PlaylistActionResult::PlaylistRenamed {
                    playlist_id,
                    name: name.trim().to_string(),
                }
            }
            Err(err) => classify_storage_error("rename_playlist", err),
        }
    }

    pub fn list_playlists_action(&self) -> Result<Vec<crate::services::storage::Playlist>> {
        self.storage.list_playlists()
    }

    pub fn list_playlist_items_action(
        &self,
        playlist_id: i64,
    ) -> Result<Vec<crate::services::storage::PlaylistItem>> {
        self.storage.load_playlist_items(playlist_id)
    }

    pub fn add_playlist_item_action(
        &mut self,
        playlist_id: i64,
        track_id: i64,
    ) -> PlaylistActionResult {
        match self.storage.add_playlist_item(playlist_id, track_id) {
            Ok(()) => {
                if let Err(err) = self.sync_active_playlist_queue_if_needed(playlist_id) {
                    return classify_storage_error("add_playlist_item", err);
                }
                PlaylistActionResult::PlaylistItemAdded {
                    playlist_id,
                    track_id,
                }
            }
            Err(err) => classify_storage_error("add_playlist_item", err),
        }
    }

    pub fn remove_playlist_item_action(
        &mut self,
        playlist_id: i64,
        order_index: i64,
    ) -> PlaylistActionResult {
        match self.storage.remove_playlist_item(playlist_id, order_index) {
            Ok(()) => {
                if let Err(err) = self.sync_active_playlist_queue_if_needed(playlist_id) {
                    return classify_storage_error("remove_playlist_item", err);
                }
                PlaylistActionResult::PlaylistItemRemoved {
                    playlist_id,
                    order_index,
                }
            }
            Err(err) => classify_storage_error("remove_playlist_item", err),
        }
    }

    pub fn play_from_playlist_action(
        &mut self,
        playlist_id: i64,
        selected_index: Option<usize>,
    ) -> PlaylistActionResult {
        let items = match self.storage.load_playlist_items(playlist_id) {
            Ok(items) => items,
            Err(err) => return classify_storage_error("play_from_playlist", err),
        };

        let start = selected_index.unwrap_or(0);
        let playable_track_id = resolve_playable_track_id(&items, start, |track_id| {
            self.track_by_id(track_id).is_some()
        });

        let Some(track_id) = playable_track_id else {
            return PlaylistActionResult::NoPlayableItem { playlist_id };
        };

        let playlist_name = self
            .storage
            .list_playlists()
            .ok()
            .and_then(|playlists| playlists.into_iter().find(|p| p.id == playlist_id))
            .map(|playlist| playlist.name)
            .unwrap_or_else(|| format!("Playlist {}", playlist_id));

        match self.activate_playlist_queue(playlist_id, playlist_name) {
            Ok(track_ids) => {
                if !track_ids.contains(&track_id) {
                    return PlaylistActionResult::NoPlayableItem { playlist_id };
                }

                match self.play_track(track_id) {
                    Ok(()) => PlaylistActionResult::PlaybackStarted {
                        playlist_id,
                        track_id,
                    },
                    Err(err) => classify_storage_error("play_from_playlist", err),
                }
            }
            Err(err) => classify_storage_error("play_from_playlist", err),
        }
    }
}

fn resolve_playable_track_id(
    items: &[crate::services::storage::PlaylistItem],
    selected_index: usize,
    mut track_exists: impl FnMut(i64) -> bool,
) -> Option<i64> {
    if items.is_empty() {
        return None;
    }

    let start = selected_index.min(items.len().saturating_sub(1));
    for item in &items[start..] {
        if item.is_missing {
            continue;
        }
        if let Some(track_id) = item.track_id
            && track_exists(track_id)
        {
            return Some(track_id);
        }
    }
    None
}

fn classify_storage_error(context: &'static str, err: anyhow::Error) -> PlaylistActionResult {
    let message = err.to_string();
    let lowered = message.to_ascii_lowercase();
    if lowered.contains("not found") {
        return PlaylistActionResult::NotFoundError { context, message };
    }
    if lowered.contains("already exists")
        || lowered.contains("cannot be empty")
        || lowered.contains("out of bounds")
    {
        return PlaylistActionResult::ValidationError { context, message };
    }
    PlaylistActionResult::PersistenceError { context, message }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::storage::PlaylistItem;

    #[test]
    fn resolve_playable_track_id_skips_missing_items() {
        let items = vec![
            PlaylistItem {
                track_id: None,
                original_path: Some(String::from("/tmp/missing.mp3")),
                is_missing: true,
                order_index: 0,
            },
            PlaylistItem {
                track_id: Some(42),
                original_path: Some(String::from("/tmp/42.mp3")),
                is_missing: false,
                order_index: 1,
            },
        ];

        let id = resolve_playable_track_id(&items, 0, |track_id| track_id == 42);
        assert_eq!(id, Some(42));
    }

    #[test]
    fn resolve_playable_track_id_returns_none_when_no_valid_track() {
        let items = vec![PlaylistItem {
            track_id: Some(9),
            original_path: Some(String::from("/tmp/9.mp3")),
            is_missing: false,
            order_index: 0,
        }];

        let id = resolve_playable_track_id(&items, 0, |_track_id| false);
        assert_eq!(id, None);
    }

    #[test]
    fn resolve_playable_track_id_honors_selected_index_start_point() {
        let items = vec![
            PlaylistItem {
                track_id: Some(1),
                original_path: Some(String::from("/tmp/1.mp3")),
                is_missing: false,
                order_index: 0,
            },
            PlaylistItem {
                track_id: Some(2),
                original_path: Some(String::from("/tmp/2.mp3")),
                is_missing: false,
                order_index: 1,
            },
        ];

        let id = resolve_playable_track_id(&items, 1, |_track_id| true);
        assert_eq!(id, Some(2));
    }

    #[test]
    fn status_mapping_for_no_playable_item_is_deterministic() {
        let result = PlaylistActionResult::NoPlayableItem { playlist_id: 1 };
        let status = result.status_message();
        assert_eq!(status.level, ActionStatusLevel::Warning);
        assert_eq!(status.code, "playlist.no_playable_item");
        assert_eq!(status.text, "No playable item found in playlist");
    }

    #[test]
    fn classify_storage_error_maps_validation_context() {
        let result = classify_storage_error(
            "create_playlist",
            anyhow::anyhow!("playlist name already exists"),
        );
        assert!(matches!(
            result,
            PlaylistActionResult::ValidationError {
                context: "create_playlist",
                ..
            }
        ));
    }
}
