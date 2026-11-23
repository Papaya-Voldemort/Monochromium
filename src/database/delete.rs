use super::Database;
use rusqlite::params_from_iter;

impl Database {
    pub fn delete_note(&self, note_ids: &[u32]) -> Result<usize, rusqlite::Error> {
        if note_ids.is_empty() {
            return Ok(0);
        }

        let placeholders = vec!["?"; note_ids.len()].join(", ");
        let out = self.conn.execute(
            &format!("DELETE FROM notes WHERE id IN ({placeholders})"),
            params_from_iter(note_ids),
        )?;

        Ok(out)
    }
}
