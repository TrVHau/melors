use std::collections::HashMap;

use crate::core::model::{PlaybackState, Track};

pub struct AppSession {
    pub tracks: Vec<Track>,
    pub track_index_by_id: HashMap<i64, usize>,
    pub tracks_version: u64,
    pub queue: Vec<i64>,
    pub queue_version: u64,
    pub playback_state: PlaybackState,
}
