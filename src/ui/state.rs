use std::cmp::min;

use crate::app::App;
use crate::core::model::Track;
use crate::features::search::search_tracks;

#[derive(Debug, Clone, Copy)]
pub enum FocusPanel {
    Sidebar,
    Library,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
}

pub struct UiState {
    pub focus: FocusPanel,
    pub mode: InputMode,
    pub search_input: String,
    pub selected: usize,
    pub status: String,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            focus: FocusPanel::Library,
            mode: InputMode::Normal,
            search_input: String::new(),
            selected: 0,
            status: String::from("Ready"),
        }
    }

    pub fn visible_tracks<'a>(&self, app: &'a App) -> Vec<&'a Track> {
        if self.mode == InputMode::Search || !self.search_input.is_empty() {
            return search_tracks(app.tracks(), &self.search_input);
        }
        app.tracks().iter().collect()
    }

    pub fn selected_track<'a>(&self, app: &'a App) -> Option<&'a Track> {
        let tracks = self.visible_tracks(app);
        if tracks.is_empty() {
            None
        } else {
            tracks.get(min(self.selected, tracks.len() - 1)).copied()
        }
    }

    pub fn selected_track_id(&self, app: &App) -> Option<i64> {
        self.selected_track(app).map(|track| track.id)
    }

    pub fn move_selection(&mut self, app: &App, delta: isize) {
        let len = self.visible_tracks(app).len();
        if len == 0 {
            self.selected = 0;
            return;
        }

        let current = self.selected as isize;
        let next = (current + delta).clamp(0, len as isize - 1);
        self.selected = next as usize;
    }
}
