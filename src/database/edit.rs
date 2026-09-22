use super::Database;
use crate::types::EditMode;
use rusqlite::params;

impl Database {
    pub fn update_row(
        &self,
        note_id: u32,
        text: String,
        mode: EditMode,
    ) -> Result<String, rusqlite::Error> {
        let pre: String = self.conn.query_row(
            "SELECT content FROM notes WHERE id = ?1",
            params![note_id],
            |row| row.get(0),
        )?;

        match mode {
            EditMode::Append => {
                self.conn.execute(
                    "UPDATE notes SET content = content || ?1 WHERE id = ?2;",
                    params![text, note_id],
                )?;
            }
            EditMode::Overwrite => {
                self.conn.execute(
                    "UPDATE notes SET content = ?1 WHERE id = ?2;",
                    params![text, note_id],
                )?;
            }
        };

        Ok(pre)
    }
}
