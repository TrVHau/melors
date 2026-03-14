use crate::core::model::{PlaybackState, Track};

pub struct AppSession {
    pub tracks: Vec<Track>,
    pub tracks_version: u64,
    pub queue: Vec<i64>,
    pub queue_version: u64,
    pub playback_state: PlaybackState,
}
