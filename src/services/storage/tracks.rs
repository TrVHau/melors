use super::*;

impl Storage {
    pub fn upsert_tracks(&mut self, inputs: &[TrackInput]) -> Result<()> {
        let tx = self.conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "
                INSERT INTO tracks (path, mtime, title, artist, album, duration)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(path) DO UPDATE SET
                    mtime=excluded.mtime,
                    title=CASE
                        WHEN tracks.title_override = 1 THEN tracks.title
                        ELSE excluded.title
                    END,
                    artist=CASE
                        WHEN tracks.artist_override = 1 THEN tracks.artist
                        ELSE excluded.artist
                    END,
                    album=CASE
                        WHEN tracks.album_override = 1 THEN tracks.album
                        ELSE excluded.album
                    END,
                    duration=excluded.duration
                WHERE
                    tracks.mtime IS NOT excluded.mtime
                    OR tracks.duration IS NOT excluded.duration
                    OR (tracks.title_override = 0 AND tracks.title IS NOT excluded.title)
                    OR (tracks.artist_override = 0 AND tracks.artist IS NOT excluded.artist)
                    OR (tracks.album_override = 0 AND tracks.album IS NOT excluded.album)
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

    pub fn rename_track(&self, track_id: i64, new_title: &str, new_path: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE tracks SET title=?1, path=?2, title_override=1 WHERE id=?3",
            params![new_title, new_path, track_id],
        )?;
        Ok(())
    }

    pub fn rename_artist(&self, track_id: i64, new_artist: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE tracks SET artist=?1, artist_override=1 WHERE id=?2",
            params![new_artist, track_id],
        )?;
        Ok(())
    }

    pub fn rename_album(&self, track_id: i64, new_album: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE tracks SET album=?1, album_override=1 WHERE id=?2",
            params![new_album, track_id],
        )?;
        Ok(())
    }
}
