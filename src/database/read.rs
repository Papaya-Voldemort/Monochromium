use super::Database;
use crate::types::{Note, NoteTypes};
use crate::utils::parse_note;
use rusqlite::params;

impl Database {
    pub fn read_rows(
        &self,
        limit: i32,
        note_type: Option<NoteTypes>,
    ) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let notes = match note_type {
            Some(t) => {
                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, content
                 FROM notes
                 WHERE type = ?1
                 ORDER BY date DESC
                 LIMIT ?2",
                )?;

                stmt.query_map(params![t.as_str(), limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, content
                 FROM notes
                 ORDER BY date DESC
                 LIMIT ?1",
                )?;

                stmt.query_map(params![limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(notes)
    }

    pub fn read_single_row(&self, note_id: u32) -> Result<Note, Box<dyn std::error::Error>> {
        let note = self.conn.query_row(
            "SELECT id, title, type, date, content
         FROM notes
         WHERE id = ?1",
            params![note_id],
            parse_note,
        )?;

        Ok(note)
    }
}
