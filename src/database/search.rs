use super::Database;
use crate::types::{Note, NoteTypes};
use crate::utils::parse_note;
use chrono::NaiveDateTime;
use rusqlite::ToSql;

impl Database {
    pub fn search_rows(
        &self,
        search: Option<String>,
        limit: u16,
        note_type: Option<NoteTypes>,
        date: Option<NaiveDateTime>,
    ) -> Result<Vec<Note>, rusqlite::Error> {
        let mut query = String::from("SELECT id, title, type, date, file_link FROM notes");
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();

        if let Some(ref s) = search
            && !s.is_empty()
        {
            conditions.push("title LIKE ?");
            params.push(Box::new(format!("%{}%", s)));
        }

        if let Some(ref t) = note_type {
            conditions.push("type = ?");
            params.push(Box::new(t.as_str().to_string()));
        }

        if let Some(d) = date {
            conditions.push("date(date) = date(?)");
            params.push(Box::new(d.format("%Y-%m-%d %H:%M:%S").to_string()));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY date DESC LIMIT ?");
        params.push(Box::new(limit));

        let mut stmt = self.conn.prepare(&query)?;

        let param_refs: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let notes = stmt
            .query_map(&param_refs[..], parse_note)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }
}
