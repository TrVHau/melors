use std::cmp::min;
use std::fmt;

use crate::app::App;
use crate::core::model::Track;
use crate::features::search::search_tracks;

#[derive(Debug, Clone, Copy)]
pub enum FocusPanel {
    Sidebar,
    Library,
    Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizerMode {
    Cava,
    Clock,
    CMatrix,
}

impl fmt::Display for VisualizerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cava => write!(f, "Cava"),
            Self::Clock => write!(f, "Clock"),
            Self::CMatrix => write!(f, "CMatrix"),
        }
    }
}

pub struct UiState {
    pub focus: FocusPanel,
    pub mode: InputMode,
    pub visualizer_mode: VisualizerMode,
    pub search_input: String,
    pub library_selected: usize,
    pub queue_selected: usize,
    pub status: String,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            focus: FocusPanel::Library,
            mode: InputMode::Normal,
            visualizer_mode: VisualizerMode::Cava,
            search_input: String::new(),
            library_selected: 0,
            queue_selected: 0,
            status: String::from("Ready"),
        }
    }

    pub fn set_visualizer_mode(&mut self, mode: VisualizerMode) {
        self.visualizer_mode = mode;
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
            tracks
                .get(min(self.library_selected, tracks.len() - 1))
                .copied()
        }
    }

    pub fn selected_track_id(&self, app: &App) -> Option<i64> {
        self.selected_track(app).map(|track| track.id)
    }

    pub fn focus_left(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Sidebar => FocusPanel::Sidebar,
            FocusPanel::Library => FocusPanel::Sidebar,
            FocusPanel::Queue => FocusPanel::Library,
        };
    }

    pub fn focus_right(&mut self) {
        self.focus = match self.focus {
            FocusPanel::Sidebar => FocusPanel::Library,
            FocusPanel::Library => FocusPanel::Queue,
            FocusPanel::Queue => FocusPanel::Queue,
        };
    }

    pub fn move_selection(&mut self, app: &App, delta: isize) {
        let len = match self.focus {
            FocusPanel::Sidebar => 0,
            FocusPanel::Library => self.visible_tracks(app).len(),
            FocusPanel::Queue => app.queue_tracks().len(),
        };

        if len == 0 {
            match self.focus {
                FocusPanel::Library => self.library_selected = 0,
                FocusPanel::Queue => self.queue_selected = 0,
                FocusPanel::Sidebar => {}
            }
            return;
        }

        let current = match self.focus {
            FocusPanel::Sidebar => 0,
            FocusPanel::Library => self.library_selected,
            FocusPanel::Queue => self.queue_selected,
        } as isize;
        let next = (current + delta).clamp(0, len as isize - 1);

        match self.focus {
            FocusPanel::Sidebar => {}
            FocusPanel::Library => self.library_selected = next as usize,
            FocusPanel::Queue => self.queue_selected = next as usize,
        }
    }
}
