use std::path::Path;

pub(super) fn music_dir_is_scannable(music_dir: &Path) -> bool {
    music_dir.exists() && music_dir.is_dir()
}

pub(super) fn is_mp3(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("mp3"))
        .unwrap_or(false)
}
