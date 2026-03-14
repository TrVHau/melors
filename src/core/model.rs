use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Track {
    pub id: i64,
    pub path: PathBuf,
    pub mtime: i64,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_secs: Option<i64>,
    pub favorite: bool,
    pub play_count: i64,
}

#[derive(Debug, Clone)]
pub struct TrackInput {
    pub path: PathBuf,
    pub mtime: i64,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration_secs: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    RepeatOne,
    RepeatAll,
}

impl RepeatMode {
    pub fn as_db_value(self) -> i64 {
        match self {
            Self::Off => 0,
            Self::RepeatOne => 1,
            Self::RepeatAll => 2,
        }
    }

    pub fn from_db_value(v: i64) -> Self {
        match v {
            1 => Self::RepeatOne,
            2 => Self::RepeatAll,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub current_track_id: Option<i64>,
    pub position_secs: i64,
    pub shuffle_enabled: bool,
    pub repeat_mode: RepeatMode,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            current_track_id: None,
            position_secs: 0,
            shuffle_enabled: false,
            repeat_mode: RepeatMode::Off,
        }
    }
}
