use crate::commands::definitions::NoteTypes;
use crate::utils::parse_note_type;
use chrono::NaiveDateTime;
use clap::builder::Str;
use libsql::params;

pub async fn add_row(
    conn: libsql::Connection,
    title: String,
    note_type: NoteTypes,
    content: String,
    date: NaiveDateTime,
) -> Result<u32, Box<dyn std::error::Error>> {
    let note_type = parse_note_type(note_type);
    let date = date.to_string();

    let output = conn
        .execute(
            "INSERT INTO notes (title, type, content, date) VALUES (?, ?, ?, ?) RETURNING id",
            params![title, note_type, content, date],
        )
        .await?;

    Ok(output as u32)
}
