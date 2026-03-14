use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use walkdir::WalkDir;

use crate::core::model::TrackInput;
use crate::services::metadata;

#[derive(Debug)]
pub struct ScanResult {
    pub upserts: Vec<TrackInput>,
    pub seen_paths: HashSet<String>,
}

pub fn scan_music_dir(music_dir: &Path) -> Result<ScanResult> {
    let mut upserts = Vec::new();
    let mut seen_paths = HashSet::new();

    if !music_dir.exists() {
        return Ok(ScanResult { upserts, seen_paths });
    }

    for entry in WalkDir::new(music_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !entry.file_type().is_file() || !is_mp3(path) {
            continue;
        }

        let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let path_text = canonical.to_string_lossy().to_string();
        seen_paths.insert(path_text);

        let metadata = fs::metadata(&canonical)?;
        let mtime = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let tag = metadata::read_metadata(&canonical);
        upserts.push(TrackInput {
            path: canonical,
            mtime,
            title: tag.title,
            artist: tag.artist,
            album: tag.album,
            duration_secs: tag.duration_secs,
        });
    }

    Ok(ScanResult { upserts, seen_paths })
}

fn is_mp3(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("mp3"))
        .unwrap_or(false)
}
