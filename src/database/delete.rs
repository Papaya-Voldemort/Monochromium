use super::Database;
use rusqlite::params;

impl Database {
    pub fn delete_note(&self, note_id: u32) -> Result<usize, Box<dyn std::error::Error>> {
        let out = self
            .conn
            .execute("DELETE FROM notes WHERE id = ?1", params![note_id])?;

        Ok(out)
    }
}
