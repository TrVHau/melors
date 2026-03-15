use super::*;

impl Storage {
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
}
