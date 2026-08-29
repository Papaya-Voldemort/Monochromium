use crate::commands::definitions::NoteTypes;
use chrono::NaiveDateTime;
use libsql::params;

pub async fn add_row(
    db: libsql::Database,
    title: String,
    note_type: NoteTypes,
    content: String,
    date: Option<NaiveDateTime>,
) {
    let mut conn = db.connect().unwrap();

    let note_type: String = match note_type {
        NoteTypes::Idea => String::from("idea"),
        NoteTypes::CheckIn => String::from("check_in"),
        NoteTypes::Random => String::from("random"),
    };
    let date = date.map(|d| d.to_string());

    conn.execute(
        "INSERT INTO notes (title, note_type, content, date) VALUES (?, ?, ?, ?)",
        params![title, note_type, content, date],
    )
    .await
    .unwrap();
}
