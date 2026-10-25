use super::Database;
use crate::types::NoteTypes;
use chrono::NaiveDateTime;
use rusqlite::params;
use std::path::PathBuf;

impl Database {
    pub fn add_row(
        &self,
        title: String,
        note_type: NoteTypes,
        file_link: PathBuf,
        date: NaiveDateTime,
        value: Option<i64>,
    ) -> Result<u32, rusqlite::Error> {
        let note_type = note_type.as_str();
        let date = date.to_string();

        let id: u32 = self.conn.query_row(
            "INSERT INTO notes (title, type, file_link, date, value)
         VALUES (?1, ?2, ?3, ?4, ?5)
         RETURNING id",
            params![title, note_type, file_link.to_str(), date, value],
            |row| row.get(0),
        )?;

        Ok(id)
    }
}
