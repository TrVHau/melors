use std::fs;

use anyhow::{Context, Result};

use super::{Config, ConfigFile};
use crate::core::config::paths::build_paths;

pub(super) fn load_or_create() -> Result<Config> {
    let paths = build_paths()?;

    fs::create_dir_all(&paths.config_dir)
        .with_context(|| format!("failed to create {}", paths.config_dir.display()))?;
    fs::create_dir_all(&paths.data_dir)
        .with_context(|| format!("failed to create {}", paths.data_dir.display()))?;
    fs::create_dir_all(&paths.cache_dir)
        .with_context(|| format!("failed to create {}", paths.cache_dir.display()))?;
    fs::create_dir_all(&paths.default_music_dir)
        .with_context(|| format!("failed to create {}", paths.default_music_dir.display()))?;

    let parsed = if paths.config_path.exists() {
        let raw = fs::read_to_string(&paths.config_path)
            .with_context(|| format!("failed to read {}", paths.config_path.display()))?;
        toml::from_str::<ConfigFile>(&raw)
            .with_context(|| format!("failed to parse {}", paths.config_path.display()))?
    } else {
        ConfigFile::default()
    };

    let music_dir = parsed.music_dir.unwrap_or(paths.default_music_dir);
    let cfg = Config {
        music_dir,
        data_dir: paths.data_dir.clone(),
        cache_dir: paths.cache_dir,
        db_path: paths.data_dir.join("db.sqlite"),
    };

    if !paths.config_path.exists() {
        let initial = ConfigFile {
            music_dir: Some(cfg.music_dir.clone()),
        };
        let toml_out = toml::to_string_pretty(&initial).context("failed to serialize config")?;
        fs::write(&paths.config_path, toml_out)
            .with_context(|| format!("failed to write {}", paths.config_path.display()))?;
    }

    Ok(cfg)
}
