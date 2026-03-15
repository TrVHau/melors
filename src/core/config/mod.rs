use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

mod load;
mod paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub music_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct ConfigFile {
    pub(super) music_dir: Option<PathBuf>,
}

impl Config {
    pub fn load_or_create() -> Result<Self> {
        load::load_or_create()
    }
}
