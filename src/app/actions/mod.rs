use std::collections::{HashMap, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::Result;

use crate::core::model::{PlaybackState, RepeatMode, Track};
use crate::services::scanner::scan_music_dir;
use crate::ui;

use super::App;
use super::state::AppSession;

mod boot;
mod library;
mod playback;
mod playlists;
mod queue;
mod rename;
mod search;
mod session;

pub(super) const PLAYBACK_PERSIST_DEBOUNCE: Duration = Duration::from_millis(250);

pub(super) fn shuffle_vec(ids: &mut [i64], current_id: Option<i64>) {
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let mut state = hasher.finish();

    let n = ids.len();
    for i in (1..n).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let j = (state as usize) % (i + 1);
        ids.swap(i, j);
    }

    // Keep the currently playing track at index 0 so next_track advances correctly.
    if let Some(id) = current_id
        && let Some(pos) = ids.iter().position(|&x| x == id)
    {
        ids.swap(0, pos);
    }
}
