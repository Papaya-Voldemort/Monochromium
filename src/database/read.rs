use super::Database;
use crate::types::{Note, NoteTypes};
use crate::utils::parse_note;
use chrono::NaiveDate;
use rusqlite::{OptionalExtension, params};
const BOUND_FORMAT: &str = "%Y-%m-%d 00:00:00";

fn to_bound(bound: Option<NaiveDate>) -> Option<String> {
    bound.map(|date| date.format(BOUND_FORMAT).to_string())
}

impl Database {
    pub fn read_rows(
        &self,
        limit: i32,
        note_type: Option<NoteTypes>,
        since: Option<NaiveDate>,
        until: Option<NaiveDate>,
    ) -> Result<Vec<Note>, rusqlite::Error> {
        let since = to_bound(since);
        let until = to_bound(until);

        let notes = match note_type {
            Some(t) => {
                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, file_link, value
                 FROM notes
                 WHERE type = ?1
                   AND (?2 IS NULL OR date >= ?2)
                   AND (?3 IS NULL OR date < ?3)
                 ORDER BY date DESC
                 LIMIT ?4",
                )?;

                stmt.query_map(params![t.as_str(), since, until, limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }
            None => {
                let mut stmt = self.conn.prepare(
                    "SELECT id, title, type, date, file_link, value
                 FROM notes
                 WHERE (?1 IS NULL OR date >= ?1)
                   AND (?2 IS NULL OR date < ?2)
                 ORDER BY date DESC
                 LIMIT ?3",
                )?;

                stmt.query_map(params![since, until, limit], parse_note)?
                    .collect::<Result<Vec<_>, _>>()?
            }
        };

        Ok(notes)
    }

    pub fn read_single_row(&self, note_id: u32) -> Result<Note, rusqlite::Error> {
        let note = self.conn.query_row(
            "SELECT id, title, type, date, file_link, value
         FROM notes
         WHERE id = ?1",
            params![note_id],
            parse_note,
        )?;

        Ok(note)
    }

    pub fn read_single_row_opt(&self, note_id: u32) -> Result<Option<Note>, rusqlite::Error> {
        let note = self
            .conn
            .query_row(
                "SELECT id, title, type, date, file_link, value
         FROM notes
         WHERE id = ?1",
                params![note_id],
                parse_note,
            )
            .optional()?;

        Ok(note)
    }

    pub fn count_rows(
        &self,
        since: Option<NaiveDate>,
    ) -> Result<Vec<(String, u32)>, rusqlite::Error> {
        let since = to_bound(since);
        let mut stmt = self.conn.prepare(
            "SELECT type, COUNT(*) FROM notes WHERE (?1 IS NULL OR date >= ?1) GROUP BY type",
        )?;

        let counts = stmt
            .query_map(params![since], |row| {
                Ok(((row.get::<_, String>(0))?, row.get::<_, u32>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(counts)
    }
}
