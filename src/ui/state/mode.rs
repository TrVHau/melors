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

    pub fn enter_rename_mode(&mut self, track_id: i64, current_title: &str) {
        self.mode = InputMode::Rename;
        self.rename_kind = RenameKind::Title;
        self.rename_track_id = Some(track_id);
        self.rename_input = current_title.to_string();
    }

    pub fn exit_rename_mode(&mut self) {
        self.mode = InputMode::Normal;
        self.rename_input.clear();
        self.rename_track_id = None;
    }

    pub fn enter_rename_artist_mode(&mut self, track_id: i64, current_artist: &str) {
        self.mode = InputMode::Rename;
        self.rename_kind = RenameKind::Artist;
        self.rename_track_id = Some(track_id);
        self.rename_input = current_artist.to_string();
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

    pub fn focus_right(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Library => FocusPanel::Queue,
            FocusPanel::Queue => FocusPanel::Library,
        };
    }

    pub fn move_selection(&mut self, app: &App, delta: isize) {
        let len = match self.focus {
            FocusPanel::Library => self.visible_track_ids(app).len(),
            FocusPanel::Queue => self.queue_track_ids(app).len(),
        };

        if len == 0 {
            match self.focus {
                FocusPanel::Library => self.library_selected = 0,
                FocusPanel::Queue => self.queue_selected = 0,
            }
            return;
        }

        let current = match self.focus {
            FocusPanel::Library => self.library_selected,
            FocusPanel::Queue => self.queue_selected,
        } as isize;
        let next = (current + delta).clamp(0, len as isize - 1);

        match self.focus {
            FocusPanel::Library => self.library_selected = next as usize,
            FocusPanel::Queue => self.queue_selected = next as usize,
        }
    }
}
