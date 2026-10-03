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
        let pre: String = match mode {
            EditMode::Title => self.conn.query_row(
                "SELECT title FROM notes WHERE id = ?1",
                params![note_id],
                |row| row.get(0),
            )?,
            EditMode::FilePath => self.conn.query_row(
                "SELECT file_link FROM notes WHERE id = ?1",
                params![note_id],
                |row| row.get(0),
            )?,
            _ => self.conn.query_row(
                "SELECT content FROM notes WHERE id = ?1",
                params![note_id],
                |row| row.get(0),
            )?,
        };

        match mode {
            EditMode::Append => {
                let full_text = format!(" {text}");

                self.conn.execute(
                    "UPDATE notes SET content = content || ?1 WHERE id = ?2;",
                    params![full_text, note_id],
                )?
            }
            EditMode::Overwrite => self.conn.execute(
                "UPDATE notes SET content = ?1 WHERE id = ?2;",
                params![text, note_id],
            )?,
            EditMode::Title => self.conn.execute(
                "UPDATE notes SET title = ?1 WHERE id = ?2",
                params![text, note_id],
            )?,
            EditMode::FilePath => self.conn.execute(
                "UPDATE notes SET file_link = ?1 WHERE id = ?2",
                params![text, note_id],
            )?,
        };

        Ok(pre)
    }
}
