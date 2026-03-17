use super::*;

#[derive(Debug, Clone, Copy)]
pub enum FocusPanel {
    Library,
    Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
    EditTag,
    PlaylistModal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaylistModalMode {
    BrowsePlaylists,
    BrowseItems,
    CreatePlaylistName,
    RenamePlaylistName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizerMode {
    Off,
    Cava,
    Clock,
    CMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiTheme {
    Neon,
    Amber,
    Mono,
    Forest,
}

impl UiTheme {
    pub fn cycle(self) -> Self {
        match self {
            Self::Neon => Self::Amber,
            Self::Amber => Self::Mono,
            Self::Mono => Self::Forest,
            Self::Forest => Self::Neon,
        }
    }
}

impl fmt::Display for UiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neon => write!(f, "Neon"),
            Self::Amber => write!(f, "Amber"),
            Self::Mono => write!(f, "Mono"),
            Self::Forest => write!(f, "Forest"),
        }
    }
}

impl fmt::Display for VisualizerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Cava => write!(f, "Cava"),
            Self::Clock => write!(f, "Clock"),
            Self::CMatrix => write!(f, "CMatrix"),
        }
    }
}

pub struct UiState {
    pub focus: FocusPanel,
    pub mode: InputMode,
    pub theme: UiTheme,
    pub visualizer_mode: VisualizerMode,
    pub visualizer_last_update_ms: u128,
    pub cava_cached_levels: Vec<(f32, f32)>,
    pub search_input: String,
    pub edit_tag_track_id: Option<i64>,
    pub edit_tag_inputs: [String; 3],
    pub edit_tag_field: usize,
    pub library_selected: usize,
    pub queue_selected: usize,
    pub status: String,
    pub search_warning: Option<String>,
    pub playlist_modal_visible: bool,
    pub playlist_modal_mode: PlaylistModalMode,
    pub playlist_name_input: String,
    pub playlist_selected: usize,
    pub playlist_item_selected: usize,
    pub playlist_action_latency_samples: u64,
    pub playlist_action_latency_total_micros: u128,
    pub playlist_action_latency_max_micros: u128,
    pub(super) library_cache_tracks_version: u64,
    pub(super) library_cache_query: String,
    pub(super) library_cache_mode: InputMode,
    pub(super) library_cache_current_track_id: Option<i64>,
    pub library_cached_track_ids: Vec<i64>,
    pub(super) library_cached_rows: Vec<String>,
    pub(super) library_render_width: usize,
    pub(super) library_cached_render_rows: Vec<String>,
    pub(super) seed_cache_valid: bool,
    pub(super) seed_cached_track_id: Option<i64>,
    pub(super) seed_cached_tracks_version: u64,
    pub(super) seed_cached_value: u64,
    pub(super) queue_cache_tracks_version: u64,
    pub(super) queue_cache_version: u64,
    pub(super) queue_cache_current_track_id: Option<i64>,
    pub(super) queue_cached_track_ids: Vec<i64>,
    pub(super) queue_cached_rows: Vec<String>,
    pub(super) queue_render_width: usize,
    pub(super) queue_cached_render_rows: Vec<String>,
}

impl UiState {
    pub fn new() -> Self {
        Self::assert_keymap_compatibility();
        Self {
            focus: FocusPanel::Library,
            mode: InputMode::Normal,
            theme: UiTheme::Neon,
            visualizer_mode: VisualizerMode::Off,
            visualizer_last_update_ms: 0,
            cava_cached_levels: Vec::new(),
            search_input: String::new(),
            edit_tag_track_id: None,
            edit_tag_inputs: [String::new(), String::new(), String::new()],
            edit_tag_field: 0,
            library_selected: 0,
            queue_selected: 0,
            status: String::from("Ready"),
            search_warning: None,
            playlist_modal_visible: false,
            playlist_modal_mode: PlaylistModalMode::BrowsePlaylists,
            playlist_name_input: String::new(),
            playlist_selected: 0,
            playlist_item_selected: 0,
            playlist_action_latency_samples: 0,
            playlist_action_latency_total_micros: 0,
            playlist_action_latency_max_micros: 0,
            library_cache_tracks_version: 0,
            library_cache_query: String::new(),
            library_cache_mode: InputMode::Normal,
            library_cache_current_track_id: None,
            library_cached_track_ids: Vec::new(),
            library_cached_rows: Vec::new(),
            library_render_width: 0,
            library_cached_render_rows: Vec::new(),
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
}
