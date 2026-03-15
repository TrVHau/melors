use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;

use crate::core::model::TrackInput;

mod io;
mod validate;

#[derive(Debug)]
pub struct ScanResult {
    pub upserts: Vec<TrackInput>,
    pub seen_paths: HashSet<String>,
}

pub fn scan_music_dir(music_dir: &Path) -> Result<ScanResult> {
    if !validate::music_dir_is_scannable(music_dir) {
        return Ok(ScanResult {
            upserts: Vec::new(),
            seen_paths: HashSet::new(),
        });
    }

    io::scan_entries(music_dir)
}
