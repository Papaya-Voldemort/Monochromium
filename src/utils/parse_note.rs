use crate::types::Note;
use chrono::NaiveDateTime;
use rusqlite::Row;

pub fn parse_note(row: &Row) -> rusqlite::Result<Note> {
    let date_str: String = row.get(3)?;

    let parsed_date =
        NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f").map_err(|err| {
            rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(err))
        })?;

    let final_date = parsed_date.format("%b %d, %Y at%l:%M %p").to_string();

    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        _note_type: row.get(2)?,
        date: final_date,
        content: row.get(4)?,
    })
}
