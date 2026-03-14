use std::cmp::min;
use std::fmt;

use crate::app::App;
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
    pub visualizer_last_update_ms: u128,
    pub cava_cached_levels: Vec<(f32, f32)>,
    pub search_input: String,
    pub library_selected: usize,
    pub queue_selected: usize,
    pub status: String,
    library_cache_tracks_version: u64,
    library_cache_query: String,
    library_cache_mode: InputMode,
    library_cache_current_track_id: Option<i64>,
    library_cached_track_ids: Vec<i64>,
    library_cached_rows: Vec<String>,
    queue_cache_tracks_version: u64,
    queue_cache_version: u64,
    queue_cache_current_track_id: Option<i64>,
    queue_cached_track_ids: Vec<i64>,
    queue_cached_rows: Vec<String>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            focus: FocusPanel::Library,
            mode: InputMode::Normal,
            visualizer_mode: VisualizerMode::Cava,
            visualizer_last_update_ms: 0,
            cava_cached_levels: Vec::new(),
            search_input: String::new(),
            library_selected: 0,
            queue_selected: 0,
            status: String::from("Ready"),
            library_cache_tracks_version: 0,
            library_cache_query: String::new(),
            library_cache_mode: InputMode::Normal,
            library_cache_current_track_id: None,
            library_cached_track_ids: Vec::new(),
            library_cached_rows: Vec::new(),
            queue_cache_tracks_version: 0,
            queue_cache_version: 0,
            queue_cache_current_track_id: None,
            queue_cached_track_ids: Vec::new(),
            queue_cached_rows: Vec::new(),
        }
    }

    pub fn set_visualizer_mode(&mut self, mode: VisualizerMode) {
        self.visualizer_mode = mode;
    }

    fn is_filtering_library(&self) -> bool {
        self.mode == InputMode::Search || !self.search_input.is_empty()
    }

    pub fn visible_track_ids(&mut self, app: &App) -> &[i64] {
        self.refresh_library_cache(app);
        &self.library_cached_track_ids
    }

    pub fn selected_track_id(&mut self, app: &App) -> Option<i64> {
        self.refresh_library_cache(app);
        if self.library_cached_track_ids.is_empty() {
            return None;
        }
        self.library_cached_track_ids
            .get(min(
                self.library_selected,
                self.library_cached_track_ids.len() - 1,
            ))
            .copied()
    }

    pub fn library_rows(&mut self, app: &App) -> &[String] {
        self.refresh_library_cache(app);
        &self.library_cached_rows
    }

    pub fn queue_track_ids(&mut self, app: &App) -> &[i64] {
        self.refresh_queue_cache(app);
        &self.queue_cached_track_ids
    }

    pub fn queue_rows(&mut self, app: &App) -> &[String] {
        self.refresh_queue_cache(app);
        &self.queue_cached_rows
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
            FocusPanel::Library => self.visible_track_ids(app).len(),
            FocusPanel::Queue => self.queue_track_ids(app).len(),
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

    fn refresh_library_cache(&mut self, app: &App) {
        let tracks_version = app.tracks_version();
        let current_track_id = app.playback_state().current_track_id;
        if self.library_cache_tracks_version == tracks_version
            && self.library_cache_query == self.search_input
            && self.library_cache_mode == self.mode
            && self.library_cache_current_track_id == current_track_id
        {
            return;
        }

        self.library_cached_track_ids.clear();
        self.library_cached_rows.clear();

        if self.is_filtering_library() {
            let tracks = search_tracks(app.tracks(), &self.search_input);
            self.library_cached_track_ids.reserve(tracks.len());
            self.library_cached_rows.reserve(tracks.len());
            for track in tracks {
                self.library_cached_track_ids.push(track.id);
                let marker = if Some(track.id) == current_track_id {
                    ">"
                } else {
                    " "
                };
                let favorite = if track.favorite { "*" } else { " " };
                self.library_cached_rows.push(format!(
                    "{}{} #{:04} {} - {}",
                    marker,
                    favorite,
                    track.id,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                ));
            }
        } else {
            let tracks = app.tracks();
            self.library_cached_track_ids.reserve(tracks.len());
            self.library_cached_rows.reserve(tracks.len());
            for track in tracks {
                self.library_cached_track_ids.push(track.id);
                let marker = if Some(track.id) == current_track_id {
                    ">"
                } else {
                    " "
                };
                let favorite = if track.favorite { "*" } else { " " };
                self.library_cached_rows.push(format!(
                    "{}{} #{:04} {} - {}",
                    marker,
                    favorite,
                    track.id,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                ));
            }
        }

        self.library_cache_tracks_version = tracks_version;
        self.library_cache_query = self.search_input.clone();
        self.library_cache_mode = self.mode;
        self.library_cache_current_track_id = current_track_id;
    }

    fn refresh_queue_cache(&mut self, app: &App) {
        let queue_version = app.queue_version();
        let tracks_version = app.tracks_version();
        let current_track_id = app.playback_state().current_track_id;
        if self.queue_cache_version == queue_version
            && self.queue_cache_tracks_version == tracks_version
            && self.queue_cache_current_track_id == current_track_id
        {
            return;
        }

        self.queue_cached_track_ids.clear();
        self.queue_cached_rows.clear();

        let queue_ids = app.queue_ids();
        self.queue_cached_track_ids.reserve(queue_ids.len());
        self.queue_cached_rows.reserve(queue_ids.len());
        for (idx, track_id) in queue_ids.iter().copied().enumerate() {
            if let Some(track) = app.track_by_id(track_id) {
                self.queue_cached_track_ids.push(track.id);
                let marker = if Some(track.id) == current_track_id {
                    ">"
                } else {
                    " "
                };
                self.queue_cached_rows.push(format!(
                    "{} {:02}. {} - {}",
                    marker,
                    idx + 1,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                    track.title
                ));
            }
        }

        self.queue_cache_version = queue_version;
        self.queue_cache_tracks_version = tracks_version;
        self.queue_cache_current_track_id = current_track_id;
    }
}
