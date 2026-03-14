use crate::core::model::{PlaybackState, Track};

pub struct AppSession {
    pub tracks: Vec<Track>,
    pub queue: Vec<i64>,
    pub playback_state: PlaybackState,
}
