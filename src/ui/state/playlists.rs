use super::*;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

const RESERVED_KEYS: &[char] = &['q', 's', 'r', 'f', 'a', 'e', 'u', 'x', 't', 'n', 'p'];
const PLAYLIST_MODAL_TOGGLE_KEY: char = 'l';
const LATENCY_LOG_SAMPLE_WINDOW: u64 = 200;
const LATENCY_LOG_PATH: &str = "/tmp/melors-ui-latency.log";

impl UiState {
    pub fn assert_keymap_compatibility() {
        debug_assert!(
            !RESERVED_KEYS.contains(&PLAYLIST_MODAL_TOGGLE_KEY),
            "playlist modal key conflicts with reserved key bindings",
        );
    }

    pub fn enter_playlist_modal(&mut self) {
        self.mode = InputMode::PlaylistModal;
        self.playlist_modal_visible = true;
        self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
        self.playlist_name_input.clear();
        self.playlist_selected = 0;
        self.playlist_item_selected = 0;
    }

    pub fn exit_playlist_modal(&mut self) {
        self.mode = InputMode::Normal;
        self.playlist_modal_visible = false;
        self.playlist_modal_mode = PlaylistModalMode::BrowsePlaylists;
        self.playlist_name_input.clear();
        self.playlist_item_selected = 0;
    }

    pub fn selected_playlist_id(&self, app: &App) -> Result<Option<i64>> {
        let playlists = app.list_playlists_action()?;
        if playlists.is_empty() {
            return Ok(None);
        }
        let idx = self.playlist_selected.min(playlists.len() - 1);
        Ok(playlists.get(idx).map(|p| p.id))
    }

    pub fn selected_playlist_item_index(&self) -> usize {
        self.playlist_item_selected
    }

    pub fn move_playlist_selection(&mut self, app: &App, delta: isize) -> Result<()> {
        let playlists = app.list_playlists_action()?;
        let len = playlists.len();
        if len == 0 {
            self.playlist_selected = 0;
            return Ok(());
        }
        let next = (self.playlist_selected as isize + delta).clamp(0, len as isize - 1);
        self.playlist_selected = next as usize;
        Ok(())
    }

    pub fn move_playlist_item_selection(&mut self, app: &App, delta: isize) -> Result<()> {
        let Some(playlist_id) = self.selected_playlist_id(app)? else {
            self.playlist_item_selected = 0;
            return Ok(());
        };
        let items = app.list_playlist_items_action(playlist_id)?;
        let len = items.len();
        if len == 0 {
            self.playlist_item_selected = 0;
            return Ok(());
        }
        let next = (self.playlist_item_selected as isize + delta).clamp(0, len as isize - 1);
        self.playlist_item_selected = next as usize;
        Ok(())
    }

    pub fn record_ui_action_latency(&mut self, elapsed_micros: u128) {
        self.playlist_action_latency_samples =
            self.playlist_action_latency_samples.saturating_add(1);
        self.playlist_action_latency_total_micros = self
            .playlist_action_latency_total_micros
            .saturating_add(elapsed_micros);
        self.playlist_action_latency_max_micros =
            self.playlist_action_latency_max_micros.max(elapsed_micros);

        if self.playlist_action_latency_samples % LATENCY_LOG_SAMPLE_WINDOW == 0 {
            let avg = self.playlist_action_latency_total_micros
                / self.playlist_action_latency_samples as u128;
            let _ = append_latency_log(format!(
                "[melors.ui.latency] samples={} avg_us={} max_us={}\n",
                self.playlist_action_latency_samples, avg, self.playlist_action_latency_max_micros
            ));
        }
    }
}

fn append_latency_log(line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LATENCY_LOG_PATH)?;
    file.write_all(line.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keymap_guard_allows_playlist_toggle_binding() {
        UiState::assert_keymap_compatibility();
    }

    #[test]
    fn playlist_modal_enter_exit_is_deterministic() {
        let mut state = UiState::new();
        state.enter_playlist_modal();
        assert_eq!(state.mode, InputMode::PlaylistModal);
        assert!(state.playlist_modal_visible);
        assert_eq!(
            state.playlist_modal_mode,
            PlaylistModalMode::BrowsePlaylists
        );

        state.exit_playlist_modal();
        assert_eq!(state.mode, InputMode::Normal);
        assert!(!state.playlist_modal_visible);
    }
}
