use std::path::PathBuf;

use anyhow::{Context, Result};

pub(super) struct AppPaths {
    pub(super) config_dir: PathBuf,
    pub(super) config_path: PathBuf,
    pub(super) data_dir: PathBuf,
    pub(super) cache_dir: PathBuf,
    pub(super) default_music_dir: PathBuf,
}

pub(super) fn build_paths() -> Result<AppPaths> {
    let home = dirs::home_dir().context("failed to resolve home directory")?;
    let config_dir = home.join(".config").join("melors");
    let data_dir = home.join(".local").join("share").join("melors");
    let cache_dir = home.join(".cache").join("melors");
    let config_path = config_dir.join("config.toml");
    let default_music_dir = home.join("Music").join("melors");

    Ok(AppPaths {
        config_dir,
        config_path,
        data_dir,
        cache_dir,
        default_music_dir,
    })
}
