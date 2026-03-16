use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Result;
use walkdir::WalkDir;

use crate::core::model::TrackInput;
use crate::services::metadata;

use super::ScanResult;
use super::validate;

pub(super) fn scan_entries(music_dir: &Path) -> Result<ScanResult> {
    let mut upserts = Vec::new();
    let mut seen_paths = HashSet::new();

    for entry in WalkDir::new(music_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !entry.file_type().is_file() || !validate::is_mp3(path) {
            continue;
        }

        let canonical = canonicalize_or_original(path);
        let path_text = canonical.to_string_lossy().to_string();
        seen_paths.insert(path_text);

        let mtime = match modified_unix_secs(&canonical) {
            Ok(value) => value,
            Err(_) => continue,
        };
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

    Ok(ScanResult {
        upserts,
        seen_paths,
    })
}

fn canonicalize_or_original(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn modified_unix_secs(path: &Path) -> Result<i64> {
    let metadata = fs::metadata(path)?;
    let mtime = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    Ok(mtime)
}
