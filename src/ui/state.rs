use std::cmp::min;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

use fuzzy_matcher::skim::SkimMatcherV2;

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
    Rename,
    EditTag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenameKind {
    Title,
    Artist,
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
    pub rename_input: String,
    pub rename_track_id: Option<i64>,
    pub rename_kind: RenameKind,
    pub edit_tag_track_id: Option<i64>,
    pub edit_tag_inputs: [String; 3],
    pub edit_tag_field: usize,
    pub library_selected: usize,
    pub queue_selected: usize,
    pub status: String,
    library_cache_tracks_version: u64,
    library_cache_query: String,
    library_cache_mode: InputMode,
    library_cache_current_track_id: Option<i64>,
    library_cached_track_ids: Vec<i64>,
    library_cached_rows: Vec<String>,
    library_render_width: usize,
    library_cached_render_rows: Vec<String>,
    matcher: SkimMatcherV2,
    seed_cache_valid: bool,
    seed_cached_track_id: Option<i64>,
    seed_cached_tracks_version: u64,
    seed_cached_value: u64,
    queue_cache_tracks_version: u64,
    queue_cache_version: u64,
    queue_cache_current_track_id: Option<i64>,
    queue_cached_track_ids: Vec<i64>,
    queue_cached_rows: Vec<String>,
    queue_render_width: usize,
    queue_cached_render_rows: Vec<String>,
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
            rename_input: String::new(),
            rename_track_id: None,
            rename_kind: RenameKind::Title,
            edit_tag_track_id: None,
            edit_tag_inputs: [String::new(), String::new(), String::new()],
            edit_tag_field: 0,
            library_selected: 0,
            queue_selected: 0,
            status: String::from("Ready"),
            library_cache_tracks_version: 0,
            library_cache_query: String::new(),
            library_cache_mode: InputMode::Normal,
            library_cache_current_track_id: None,
            library_cached_track_ids: Vec::new(),
            library_cached_rows: Vec::new(),
            library_render_width: 0,
            library_cached_render_rows: Vec::new(),
            matcher: SkimMatcherV2::default(),
            seed_cache_valid: false,
            seed_cached_track_id: None,
            seed_cached_tracks_version: 0,
            seed_cached_value: 0,
            queue_cache_tracks_version: 0,
            queue_cache_version: 0,
            queue_cache_current_track_id: None,
            queue_cached_track_ids: Vec::new(),
            queue_cached_rows: Vec::new(),
            queue_render_width: 0,
            queue_cached_render_rows: Vec::new(),
        }
    }

    pub fn set_visualizer_mode(&mut self, mode: VisualizerMode) {
        self.visualizer_mode = mode;
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = InputMode::Search;
        self.search_input.clear();
        self.library_selected = 0;
        self.invalidate_library_cache();
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = InputMode::Normal;
        self.search_input.clear();
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

    fn is_filtering_library(&self) -> bool {
        self.mode == InputMode::Search
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

    pub fn library_rows_for_width(&mut self, app: &App, width: usize) -> &[String] {
        self.refresh_library_cache(app);
        if self.library_render_width != width
            || self.library_cached_render_rows.len() != self.library_cached_rows.len()
        {
            self.library_cached_render_rows = self
                .library_cached_rows
                .iter()
                .map(|row| Self::fixed_width_cell(row, width))
                .collect();
            self.library_render_width = width;
        }
        &self.library_cached_render_rows
    }

    pub fn queue_track_ids(&mut self, app: &App) -> &[i64] {
        self.refresh_queue_cache(app);
        &self.queue_cached_track_ids
    }

    pub fn queue_rows_for_width(&mut self, app: &App, width: usize) -> &[String] {
        self.refresh_queue_cache(app);
        if self.queue_render_width != width
            || self.queue_cached_render_rows.len() != self.queue_cached_rows.len()
        {
            self.queue_cached_render_rows = self
                .queue_cached_rows
                .iter()
                .map(|row| Self::fixed_width_cell(row, width))
                .collect();
            self.queue_render_width = width;
        }
        &self.queue_cached_render_rows
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
            let tracks = search_tracks(&self.matcher, app.tracks(), &self.search_input);
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
        self.library_render_width = 0;
        self.library_cached_render_rows.clear();
    }

    fn invalidate_library_cache(&mut self) {
        self.library_cache_tracks_version = 0;
        self.library_cache_query.clear();
        self.library_cache_current_track_id = None;
        self.library_cached_track_ids.clear();
        self.library_cached_rows.clear();
        self.library_render_width = 0;
        self.library_cached_render_rows.clear();
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
        self.queue_render_width = 0;
        self.queue_cached_render_rows.clear();
    }

    pub fn cached_track_seed(&mut self, app: &App) -> u64 {
        let current_track_id = app.playback_state().current_track_id;
        let tracks_version = app.tracks_version();
        if self.seed_cache_valid
            && self.seed_cached_track_id == current_track_id
            && self.seed_cached_tracks_version == tracks_version
        {
            return self.seed_cached_value;
        }
        let mut hasher = DefaultHasher::new();
        if let Some(track) = app.current_track() {
            track.id.hash(&mut hasher);
            track.path.hash(&mut hasher);
            track.title.hash(&mut hasher);
            track.artist.hash(&mut hasher);
            track.album.hash(&mut hasher);
            track.duration_secs.hash(&mut hasher);
        } else {
            0u64.hash(&mut hasher);
        }
        let value = hasher.finish();
        self.seed_cache_valid = true;
        self.seed_cached_track_id = current_track_id;
        self.seed_cached_tracks_version = tracks_version;
        self.seed_cached_value = value;
        value
    }

    fn fixed_width_cell(text: &str, width: usize) -> String {
        if width == 0 {
            return String::new();
        }

        let mut out: String = text.chars().take(width).collect();
        let used = out.chars().count();
        if used < width {
            out.push_str(&" ".repeat(width - used));
        }
        out
    }
}
