use std::collections::HashSet;

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};

use crate::core::model::{PlaybackState, RepeatMode, Track, TrackInput};

mod migrations;
mod playback;
mod queue;
mod tracks;

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn open(db_path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let storage = Self { conn };
        storage.run_migrations()?;
        Ok(storage)
    }
}
