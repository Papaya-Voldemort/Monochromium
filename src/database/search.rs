use super::Database;
use crate::types::{Note, NoteTypes};
use crate::utils::parse_note;
use rusqlite::params;

impl Database {
    pub fn search_rows(
        &self,
        search: Option<String>,
        limit: u16,
        note_type: Option<NoteTypes>,
    ) -> Result<Vec<Note>, rusqlite::Error> {
        let notes = match (search, note_type) {
            (Some(search), _) if !search.is_empty() => {
                let pattern = format!("%{search}%");

                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, content
                 FROM notes
                 WHERE title LIKE ?1 OR content LIKE ?1
                 ORDER BY date DESC
                 LIMIT ?2",
                )?;

                stmt.query_map(params![pattern, limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }
            (_, Some(note_type)) => {
                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, content
                 FROM notes
                 WHERE type = ?1
                 ORDER BY date DESC
                 LIMIT ?2",
                )?;

                stmt.query_map(params![note_type.as_str(), limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }

            _ => {
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
}
