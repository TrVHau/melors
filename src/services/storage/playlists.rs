#![allow(dead_code)]

use super::*;

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub track_id: Option<i64>,
    pub original_path: Option<String>,
    pub is_missing: bool,
    pub order_index: i64,
}

impl Storage {
    pub fn create_playlist(&self, name: &str) -> Result<i64> {
        let normalized = normalize_playlist_name(name)?;
        if playlist_name_exists(&self.conn, &normalized, None)? {
            return Err(anyhow::anyhow!("playlist name already exists"));
        }
        self.conn.execute(
            "INSERT INTO playlists (name) VALUES (?1)",
            params![normalized],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn rename_playlist(&self, playlist_id: i64, name: &str) -> Result<()> {
        let normalized = normalize_playlist_name(name)?;
        if playlist_name_exists(&self.conn, &normalized, Some(playlist_id))? {
            return Err(anyhow::anyhow!("playlist name already exists"));
        }

        let affected = self.conn.execute(
            "UPDATE playlists SET name=?1 WHERE id=?2",
            params![normalized, playlist_id],
        )?;
        if affected == 0 {
            return Err(anyhow::anyhow!("playlist not found"));
        }
        Ok(())
    }

    pub fn delete_playlist(&mut self, playlist_id: i64) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "DELETE FROM playlist_items WHERE playlist_id=?1",
            params![playlist_id],
        )?;
        let affected = tx.execute("DELETE FROM playlists WHERE id=?1", params![playlist_id])?;
        if affected == 0 {
            return Err(anyhow::anyhow!("playlist not found"));
        }
        tx.commit()?;
        Ok(())
    }

    pub fn list_playlists(&self) -> Result<Vec<Playlist>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at FROM playlists ORDER BY name COLLATE NOCASE ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Playlist {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn add_playlist_item(&mut self, playlist_id: i64, track_id: i64) -> Result<()> {
        let tx = self.conn.transaction()?;
        ensure_playlist_exists_tx(&tx, playlist_id)?;
        let track_path: String = tx
            .query_row(
                "SELECT path FROM tracks WHERE id=?1",
                params![track_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| anyhow::anyhow!("track not found"))?;

        let next_order: i64 = tx.query_row(
            "SELECT COALESCE(MAX(order_index), -1) + 1 FROM playlist_items WHERE playlist_id=?1",
            params![playlist_id],
            |row| row.get(0),
        )?;

        tx.execute(
            "INSERT INTO playlist_items (playlist_id, track_id, original_path, is_missing, order_index) VALUES (?1, ?2, ?3, 0, ?4)",
            params![playlist_id, track_id, track_path, next_order],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn remove_playlist_item(&mut self, playlist_id: i64, order_index: i64) -> Result<()> {
        let tx = self.conn.transaction()?;
        ensure_playlist_exists_tx(&tx, playlist_id)?;

        let affected = tx.execute(
            "DELETE FROM playlist_items WHERE rowid IN (
                SELECT rowid FROM playlist_items WHERE playlist_id=?1 AND order_index=?2 LIMIT 1
            )",
            params![playlist_id, order_index],
        )?;
        if affected == 0 {
            return Err(anyhow::anyhow!("playlist item not found"));
        }

        tx.execute(
            "UPDATE playlist_items SET order_index = order_index - 1
             WHERE playlist_id=?1 AND order_index > ?2",
            params![playlist_id, order_index],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn move_playlist_item(
        &mut self,
        playlist_id: i64,
        from_index: i64,
        delta: isize,
    ) -> Result<i64> {
        let tx = self.conn.transaction()?;
        ensure_playlist_exists_tx(&tx, playlist_id)?;

        let count: i64 = tx.query_row(
            "SELECT COUNT(*) FROM playlist_items WHERE playlist_id=?1",
            params![playlist_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(anyhow::anyhow!("playlist has no items"));
        }
        if from_index < 0 || from_index >= count {
            return Err(anyhow::anyhow!("source index out of bounds"));
        }

        let to_index = (from_index + delta as i64).clamp(0, count - 1);
        if to_index == from_index {
            tx.commit()?;
            return Ok(to_index);
        }

        let row_id: i64 = tx
            .query_row(
                "SELECT rowid FROM playlist_items WHERE playlist_id=?1 AND order_index=?2 LIMIT 1",
                params![playlist_id, from_index],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| anyhow::anyhow!("playlist item not found"))?;

        if to_index > from_index {
            tx.execute(
                "UPDATE playlist_items
                 SET order_index = order_index - 1
                 WHERE playlist_id=?1 AND order_index > ?2 AND order_index <= ?3",
                params![playlist_id, from_index, to_index],
            )?;
        } else {
            tx.execute(
                "UPDATE playlist_items
                 SET order_index = order_index + 1
                 WHERE playlist_id=?1 AND order_index >= ?2 AND order_index < ?3",
                params![playlist_id, to_index, from_index],
            )?;
        }

        tx.execute(
            "UPDATE playlist_items SET order_index=?1 WHERE rowid=?2",
            params![to_index, row_id],
        )?;
        tx.commit()?;
        Ok(to_index)
    }

    pub fn load_playlist_items(&self, playlist_id: i64) -> Result<Vec<PlaylistItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT track_id, original_path, is_missing, order_index
             FROM playlist_items
             WHERE playlist_id=?1
             ORDER BY order_index ASC",
        )?;
        let rows = stmt.query_map(params![playlist_id], |row| {
            Ok(PlaylistItem {
                track_id: row.get(0)?,
                original_path: row.get(1)?,
                is_missing: row.get::<_, i64>(2)? != 0,
                order_index: row.get(3)?,
            })
        })?;

        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }
}

#[allow(dead_code)]
fn normalize_playlist_name(name: &str) -> Result<String> {
    let normalized = name.trim();
    if normalized.is_empty() {
        return Err(anyhow::anyhow!("playlist name cannot be empty"));
    }
    Ok(normalized.to_string())
}

#[allow(dead_code)]
fn playlist_name_exists(conn: &Connection, name: &str, exclude_id: Option<i64>) -> Result<bool> {
    let exists = match exclude_id {
        Some(id) => conn
            .query_row(
                "SELECT 1 FROM playlists WHERE name = ?1 COLLATE NOCASE AND id != ?2 LIMIT 1",
                params![name, id],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .is_some(),
        None => conn
            .query_row(
                "SELECT 1 FROM playlists WHERE name = ?1 COLLATE NOCASE LIMIT 1",
                params![name],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .is_some(),
    };
    Ok(exists)
}

#[allow(dead_code)]
fn ensure_playlist_exists_tx(tx: &rusqlite::Transaction<'_>, playlist_id: i64) -> Result<()> {
    let exists = tx
        .query_row(
            "SELECT 1 FROM playlists WHERE id=?1 LIMIT 1",
            params![playlist_id],
            |row| row.get::<_, i64>(0),
        )
        .optional()?
        .is_some();
    if !exists {
        return Err(anyhow::anyhow!("playlist not found"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("melors-playlists-test-{nanos}.sqlite"))
    }

    fn cleanup_db(path: &PathBuf) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(format!("{}-wal", path.to_string_lossy()));
        let _ = fs::remove_file(format!("{}-shm", path.to_string_lossy()));
    }

    fn seed_tracks(storage: &mut Storage) {
        storage
            .upsert_tracks(&[
                TrackInput {
                    path: PathBuf::from("/tmp/p1.mp3"),
                    mtime: 1,
                    title: "Track 1".to_string(),
                    artist: Some("Artist".to_string()),
                    album: Some("Album".to_string()),
                    duration_secs: Some(100),
                },
                TrackInput {
                    path: PathBuf::from("/tmp/p2.mp3"),
                    mtime: 1,
                    title: "Track 2".to_string(),
                    artist: Some("Artist".to_string()),
                    album: Some("Album".to_string()),
                    duration_secs: Some(110),
                },
            ])
            .expect("seed tracks");
    }

    #[test]
    fn create_playlist_enforces_case_insensitive_uniqueness() {
        let db_path = temp_db_path();
        let storage = Storage::open(&db_path).expect("open storage");

        storage.create_playlist("Focus").expect("create playlist");
        let err = storage.create_playlist("focus").expect_err("must conflict");
        assert!(err.to_string().contains("already exists"));

        drop(storage);
        cleanup_db(&db_path);
    }

    #[test]
    fn move_playlist_item_clamps_and_keeps_contiguous_order() {
        let db_path = temp_db_path();
        let mut storage = Storage::open(&db_path).expect("open storage");
        seed_tracks(&mut storage);
        let playlist_id = storage.create_playlist("Queue").expect("create playlist");
        let tracks = storage.load_tracks().expect("load tracks");

        storage
            .add_playlist_item(playlist_id, tracks[0].id)
            .expect("add 1");
        storage
            .add_playlist_item(playlist_id, tracks[1].id)
            .expect("add 2");

        let moved = storage
            .move_playlist_item(playlist_id, 0, 999)
            .expect("move");
        assert_eq!(moved, 1);

        let items = storage
            .load_playlist_items(playlist_id)
            .expect("load items");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].order_index, 0);
        assert_eq!(items[1].order_index, 1);

        drop(storage);
        cleanup_db(&db_path);
    }

    #[test]
    fn failed_playlist_mutation_rolls_back_changes() {
        let db_path = temp_db_path();
        let mut storage = Storage::open(&db_path).expect("open storage");
        seed_tracks(&mut storage);
        let playlist_id = storage.create_playlist("Safe").expect("create playlist");
        let track_id = storage.load_tracks().expect("load tracks")[0].id;
        storage
            .add_playlist_item(playlist_id, track_id)
            .expect("seed item");

        let err = storage
            .remove_playlist_item(playlist_id, 9)
            .expect_err("must fail");
        assert!(err.to_string().contains("not found"));

        let items = storage
            .load_playlist_items(playlist_id)
            .expect("load items");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].order_index, 0);

        drop(storage);
        cleanup_db(&db_path);
    }
}
