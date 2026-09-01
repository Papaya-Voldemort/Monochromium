use crate::commands::definitions::NoteTypes;
use chrono::NaiveDateTime;
use libsql::params;
use crate::utils::parse_note_type;

pub async fn add_row(
    db: libsql::Database,
    title: String,
    note_type: NoteTypes,
    content: String,
    date: NaiveDateTime,
) {
    let conn = db.connect().unwrap();

    let note_type = parse_note_type(note_type);
    let date = date.to_string();

    conn.execute(
        "INSERT INTO notes (title, type, content, date) VALUES (?, ?, ?, ?)",
        params![title, note_type, content, date],
    )
    .await
    .unwrap();
}
