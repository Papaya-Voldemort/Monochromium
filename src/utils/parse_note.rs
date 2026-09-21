use crate::types::Note;
use chrono::NaiveDateTime;
use rusqlite::Row;

pub fn parse_note(row: &Row) -> rusqlite::Result<Note> {
    let date_str: String = row.get(3)?;

    let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f").map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(err))
    })?;

    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        _note_type: row.get(2)?,
        date,
        content: row.get(4)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    #[test]
    fn test_note_parsing() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "
            CREATE TABLE notes (
                id INTEGER,
                title TEXT,
                type TEXT,
                date TEXT,
                content TEXT
            )
            ",
            [],
        )
        .unwrap();

        conn.execute(
            "
            INSERT INTO notes (id, title, type, date, content)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![1, "Test note", "note", "2026-09-21 09:30:00", "Hello world"],
        )
        .unwrap();

        let note = conn
            .query_row(
                "SELECT id, title, type, date, content FROM notes WHERE id = 1",
                [],
                parse_note,
            )
            .unwrap();

        assert_eq!(note.id, 1);
        assert_eq!(note.title, "Test note");
        assert_eq!(note._note_type, "note");
        assert_eq!(note.content, "Hello world");

        assert_eq!(
            note.date,
            NaiveDateTime::parse_from_str("2026-09-21 09:30:00", "%Y-%m-%d %H:%M:%S%.f").unwrap()
        );
    }
}
