use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::Result;
use walkdir::WalkDir;

use crate::core::model::TrackInput;
use crate::services::metadata;

use super::ScanResult;
use super::ScanWarningAggregate;
use super::validate;

pub(super) fn scan_entries(music_dir: &Path) -> Result<ScanResult> {
    let mut upserts = Vec::new();
    let mut seen_paths = HashSet::new();
    let mut warnings = ScanWarningAggregate::default();

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
        seen_paths.insert(path_text.clone());

        match build_track_input(canonical) {
            Ok(input) => upserts.push(input),
            Err(err) => {
                warnings.failed_files = warnings.failed_files.saturating_add(1);
                if warnings.failed_paths_sample.len() < 5 {
                    warnings
                        .failed_paths_sample
                        .push(format!("{}: {}", path_text, err));
                }
            }
        }
    }

    Ok(ScanResult {
        upserts,
        seen_paths,
        warnings,
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

fn build_track_input(path: PathBuf) -> std::result::Result<TrackInput, String> {
    let mtime = modified_unix_secs(&path).map_err(|e| e.to_string())?;
    let tag = metadata::read_metadata(&path);
    Ok(TrackInput {
        path,
        mtime,
        title: tag.title,
        artist: tag.artist,
        album: tag.album,
        duration_secs: tag.duration_secs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_track_input_returns_none_for_missing_file() {
        let missing = PathBuf::from("/tmp/melors-missing-file-for-scan-test.mp3");
        assert!(build_track_input(missing).is_err());
    }

    #[test]
    fn build_track_input_reads_minimal_mp3_like_file() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("melors-scan-test-{unique}.mp3"));
        std::fs::write(&path, b"not-a-real-mp3").expect("write temp file");

        let input = build_track_input(path.clone());
        assert!(input.is_ok());
        let input = input.expect("track input should exist");
        assert_eq!(input.path, path);
        assert!(!input.title.is_empty());

        let _ = std::fs::remove_file(path);
    }
}
