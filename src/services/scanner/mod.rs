use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;

use crate::core::model::TrackInput;

mod io;
mod validate;
mod watcher;

pub use watcher::{WatcherEventBatch, spawn_watcher_runtime};

#[derive(Debug, Default, Clone)]
pub struct ScanWarningAggregate {
    pub failed_files: usize,
    pub failed_paths_sample: Vec<String>,
}

#[derive(Debug)]
pub struct ScanResult {
    pub upserts: Vec<TrackInput>,
    pub seen_paths: HashSet<String>,
    pub warnings: ScanWarningAggregate,
}

pub fn scan_music_dir(music_dir: &Path) -> Result<ScanResult> {
    if !validate::music_dir_is_scannable(music_dir) {
        return Ok(ScanResult {
            upserts: Vec::new(),
            seen_paths: HashSet::new(),
            warnings: ScanWarningAggregate::default(),
        });
    }

    io::scan_entries(music_dir)
}
