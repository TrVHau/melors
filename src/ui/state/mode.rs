use super::*;

impl UiState {
    pub fn set_visualizer_mode(&mut self, mode: VisualizerMode) {
        self.visualizer_mode = mode;
    }

    pub fn cycle_theme(&mut self) -> UiTheme {
        self.theme = self.theme.cycle();
        self.theme
    }

    pub fn enter_search_mode(&mut self) {
        self.focus = FocusPanel::Library;
        self.mode = InputMode::Search;
        self.search_input.clear();
        self.search_warning = None;
        self.library_selected = 0;
        self.invalidate_library_cache();
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = InputMode::Normal;
        self.search_input.clear();
        self.search_warning = None;
        self.library_selected = 0;
        self.invalidate_library_cache();
    }

    pub fn enter_edit_tag_mode(&mut self, track_id: i64, title: &str, artist: &str, album: &str) {
        self.mode = InputMode::EditTag;
        self.edit_tag_track_id = Some(track_id);
        self.edit_tag_inputs = [title.to_string(), artist.to_string(), album.to_string()];
        self.edit_tag_field = 0;
    }

    pub fn exit_edit_tag_mode(&mut self) {
        self.mode = InputMode::Normal;
        self.edit_tag_track_id = None;
        self.edit_tag_inputs = [String::new(), String::new(), String::new()];
        self.edit_tag_field = 0;
    }

    pub fn move_selection(&mut self, app: &App, delta: isize) {
        let len = self.visible_track_ids(app).len();

        if len == 0 {
            self.library_selected = 0;
            return;
        }

        let current = self.library_selected as isize;
        let next = (current + delta).clamp(0, len as isize - 1);
        self.library_selected = next as usize;
    }
}
