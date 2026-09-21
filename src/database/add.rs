use super::Database;
use crate::types::NoteTypes;
use chrono::NaiveDateTime;
use rusqlite::params;

impl Database {
    pub fn add_row(
        &self,
        title: String,
        note_type: NoteTypes,
        content: String,
        date: NaiveDateTime,
    ) -> Result<u32, rusqlite::Error> {
        let note_type = note_type.as_str();
        let date = date.to_string();

        let id: u32 = self.conn.query_row(
            "INSERT INTO notes (title, type, content, date)
         VALUES (?1, ?2, ?3, ?4)
         RETURNING id",
            params![title, note_type, content, date],
            |row| row.get(0),
        )?;

        Ok(id)
    }
}
