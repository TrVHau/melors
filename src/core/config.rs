use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub music_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ConfigFile {
    music_dir: Option<PathBuf>,
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        let home = dirs::home_dir().context("failed to resolve home directory")?;
        let config_dir = home.join(".config").join("melors");
        let data_dir = home.join(".local").join("share").join("melors");
        let cache_dir = home.join(".cache").join("melors");
        let config_path = config_dir.join("config.toml");

        fs::create_dir_all(&config_dir)
            .with_context(|| format!("failed to create {}", config_dir.display()))?;
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("failed to create {}", data_dir.display()))?;
        fs::create_dir_all(&cache_dir)
            .with_context(|| format!("failed to create {}", cache_dir.display()))?;

        let default_music_dir = home.join("Music").join("melors");
        let parsed = if config_path.exists() {
            let raw = fs::read_to_string(&config_path)
                .with_context(|| format!("failed to read {}", config_path.display()))?;
            toml::from_str::<ConfigFile>(&raw)
                .with_context(|| format!("failed to parse {}", config_path.display()))?
        } else {
            ConfigFile::default()
        };

        let music_dir = parsed.music_dir.unwrap_or(default_music_dir);
        let cfg = Self {
            music_dir,
            data_dir: data_dir.clone(),
            cache_dir,
            db_path: data_dir.join("db.sqlite"),
        };

        if !config_path.exists() {
            let initial = ConfigFile {
                music_dir: Some(cfg.music_dir.clone()),
            };
            let toml_out =
                toml::to_string_pretty(&initial).context("failed to serialize config")?;
            fs::write(&config_path, toml_out)
                .with_context(|| format!("failed to write {}", config_path.display()))?;
        }

        Ok(cfg)
    }
}
