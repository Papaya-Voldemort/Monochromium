use crate::commands::definitions::NoteTypes;
use chrono::NaiveDateTime;
use libsql::params;

pub async fn add_row(
    conn: libsql::Connection,
    title: String,
    note_type: NoteTypes,
    content: String,
    date: NaiveDateTime,
) -> Result<u32, Box<dyn std::error::Error>> {
    let note_type = note_type.as_str();
    let date = date.to_string();

    let mut rows = conn
        .query(
            "INSERT INTO notes (title, type, content, date) VALUES (?, ?, ?, ?) RETURNING id",
            params![title, note_type, content, date],
        )
        .await?;

    let row = rows.next().await?.unwrap();
    let id: u32 = row.get(0)?;

    Ok(id)
}
