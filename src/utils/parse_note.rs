use crate::types::{Note, NoteTypes};
use chrono::NaiveDateTime;
use rusqlite::Row;
use std::path::PathBuf;

pub fn parse_note(row: &Row) -> Result<Note, rusqlite::Error> {
    let date_str: String = row.get(3)?;
    let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f").map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(err))
    })?;

    let note_type_raw: String = row.get(2)?;
    let note_type: NoteTypes = note_type_raw.parse().map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(err))
    })?;

    let file_link_raw: String = row.get(4)?;
    let file_link = PathBuf::from(file_link_raw);

    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        note_type,
        date,
        file_link,
        content: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MonoError;
    use rusqlite::{Connection, params};

    #[test]
    fn test_note_parsing() -> Result<(), MonoError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "
            CREATE TABLE notes (
                id INTEGER,
                title TEXT,
                type TEXT,
                date TEXT,
                file_link TEXT
            )
            ",
            [],
        )?;

        conn.execute(
            "
            INSERT INTO notes (id, title, type, date, file_link)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![
                1,
                "Test note",
                "other",
                "2026-09-21 09:30:00",
                "path/note.md"
            ],
        )?;

        let note = conn.query_row(
            "SELECT id, title, type, date, file_link FROM notes WHERE id = 1",
            [],
            parse_note,
        )?;

        assert_eq!(note.id, 1);
        assert_eq!(note.title, "Test note");
        assert_eq!(note.note_type, NoteTypes::Other);
        assert_eq!(note.file_link, PathBuf::from("path/note.md"));
        assert_eq!(note.content, None);

        assert_eq!(
            note.date,
            NaiveDateTime::parse_from_str("2026-09-21 09:30:00", "%Y-%m-%d %H:%M:%S%.f").unwrap()
        );

        Ok(())
    }
}
