use std::collections::HashSet;

use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};

use crate::core::model::{PlaybackState, RepeatMode, Track, TrackInput};

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

    fn run_migrations(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                mtime INTEGER NOT NULL,
                title TEXT NOT NULL,
                artist TEXT,
                album TEXT,
                duration INTEGER,
                favorite INTEGER NOT NULL DEFAULT 0,
                play_count INTEGER NOT NULL DEFAULT 0,
                last_played_at TEXT
            );

            CREATE TABLE IF NOT EXISTS queue_state (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                position INTEGER NOT NULL,
                track_id INTEGER NOT NULL,
                FOREIGN KEY(track_id) REFERENCES tracks(id)
            );

            CREATE TABLE IF NOT EXISTS playback_state (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                current_track_id INTEGER,
                position_secs INTEGER NOT NULL DEFAULT 0,
                shuffle_enabled INTEGER NOT NULL DEFAULT 0,
                repeat_mode INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(current_track_id) REFERENCES tracks(id)
            );

            CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS playlist_items (
                playlist_id INTEGER NOT NULL,
                track_id INTEGER,
                original_path TEXT,
                is_missing INTEGER NOT NULL DEFAULT 0,
                order_index INTEGER NOT NULL,
                FOREIGN KEY(playlist_id) REFERENCES playlists(id),
                FOREIGN KEY(track_id) REFERENCES tracks(id)
            );
            ",
        )?;
        self.conn.execute(
            "INSERT OR IGNORE INTO playback_state (id, current_track_id, position_secs, shuffle_enabled, repeat_mode) VALUES (1, NULL, 0, 0, 0)",
            [],
        )?;
        Ok(())
    }

    pub fn upsert_tracks(&mut self, inputs: &[TrackInput]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "
                INSERT INTO tracks (path, mtime, title, artist, album, duration)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(path) DO UPDATE SET
                    mtime=excluded.mtime,
                    title=excluded.title,
                    artist=excluded.artist,
                    album=excluded.album,
                    duration=excluded.duration
                ",
            )?;

            for t in inputs {
                stmt.execute(params![
                    t.path.to_string_lossy().to_string(),
                    t.mtime,
                    t.title,
                    t.artist,
                    t.album,
                    t.duration_secs,
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn prune_missing_tracks(&mut self, seen_paths: &HashSet<String>) -> Result<usize> {
        let mut stale_ids = Vec::new();
        {
            let mut stmt = self.conn.prepare("SELECT id, path FROM tracks")?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let path: String = row.get(1)?;
                Ok((id, path))
            })?;

            for row in rows {
                let (id, path) = row?;
                if !seen_paths.contains(&path) {
                    stale_ids.push(id);
                }
            }
        }

        let tx = self.conn.transaction()?;
        for id in &stale_ids {
            tx.execute("DELETE FROM queue_state WHERE track_id=?1", params![id])?;
            tx.execute(
                "UPDATE playback_state SET current_track_id=NULL, position_secs=0 WHERE current_track_id=?1",
                params![id],
            )?;
            tx.execute("DELETE FROM tracks WHERE id=?1", params![id])?;
            tx.execute(
                "UPDATE playlist_items SET track_id=NULL, is_missing=1 WHERE track_id=?1",
                params![id],
            )?;
        }
        tx.commit()?;

        Ok(stale_ids.len())
    }

    pub fn load_tracks(&self) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT id, path, mtime, title, artist, album, duration, favorite, play_count
            FROM tracks
            ORDER BY artist COLLATE NOCASE ASC, album COLLATE NOCASE ASC, title COLLATE NOCASE ASC
            ",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Track {
                id: row.get(0)?,
                path: row.get::<_, String>(1)?.into(),
                mtime: row.get(2)?,
                title: row.get(3)?,
                artist: row.get(4)?,
                album: row.get(5)?,
                duration_secs: row.get(6)?,
                favorite: row.get::<_, i64>(7)? != 0,
                play_count: row.get(8)?,
            })
        })?;

        let mut tracks = Vec::new();
        for row in rows {
            tracks.push(row?);
        }
        Ok(tracks)
    }

    pub fn save_playback_state(&self, state: &PlaybackState) -> Result<()> {
        self.conn.execute(
            "
            UPDATE playback_state
            SET current_track_id=?1, position_secs=?2, shuffle_enabled=?3, repeat_mode=?4, updated_at=CURRENT_TIMESTAMP
            WHERE id=1
            ",
            params![
                state.current_track_id,
                state.position_secs,
                i64::from(state.shuffle_enabled),
                state.repeat_mode.as_db_value()
            ],
        )?;
        Ok(())
    }

    pub fn load_playback_state(&self) -> Result<PlaybackState> {
        let state = self
            .conn
            .query_row(
                "
                SELECT current_track_id, position_secs, shuffle_enabled, repeat_mode
                FROM playback_state
                WHERE id=1
                ",
                [],
                |row| {
                    Ok(PlaybackState {
                        current_track_id: row.get(0)?,
                        position_secs: row.get(1)?,
                        shuffle_enabled: row.get::<_, i64>(2)? != 0,
                        repeat_mode: RepeatMode::from_db_value(row.get::<_, i64>(3)?),
                    })
                },
            )
            .optional()?;
        Ok(state.unwrap_or_default())
    }

    pub fn replace_queue(&mut self, track_ids: &[i64]) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM queue_state", [])?;
        {
            let mut stmt =
                tx.prepare("INSERT INTO queue_state (position, track_id) VALUES (?1, ?2)")?;
            for (idx, id) in track_ids.iter().enumerate() {
                stmt.execute(params![idx as i64, id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn load_queue(&self) -> Result<Vec<i64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT track_id FROM queue_state ORDER BY position ASC")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn increment_play_count(&self, track_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE tracks SET play_count = play_count + 1, last_played_at = CURRENT_TIMESTAMP WHERE id=?1",
            params![track_id],
        )?;
        Ok(())
    }

    pub fn toggle_favorite(&self, track_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE tracks SET favorite = CASE favorite WHEN 0 THEN 1 ELSE 0 END WHERE id=?1",
            params![track_id],
        )?;
        Ok(())
    }
}
