use super::*;

impl Storage {
    pub(super) fn run_migrations(&self) -> Result<()> {
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
                title_override INTEGER NOT NULL DEFAULT 0,
                artist_override INTEGER NOT NULL DEFAULT 0,
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

        // Backfill override columns for older databases.
        let _ = self.conn.execute(
            "ALTER TABLE tracks ADD COLUMN title_override INTEGER NOT NULL DEFAULT 0",
            [],
        );
        let _ = self.conn.execute(
            "ALTER TABLE tracks ADD COLUMN artist_override INTEGER NOT NULL DEFAULT 0",
            [],
        );
        let _ = self.conn.execute(
            "ALTER TABLE tracks ADD COLUMN album_override INTEGER NOT NULL DEFAULT 0",
            [],
        );
        Ok(())
    }
}
