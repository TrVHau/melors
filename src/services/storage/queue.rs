use super::*;

impl Storage {
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
}
