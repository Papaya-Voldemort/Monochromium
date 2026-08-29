use crate::commands::definitions::NoteTypes;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Local};
use crate::database::add_row;


pub async fn add(
    db: libsql::Database,
    text: String,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) {
    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());

    let title = text.clone().trim().to_string();
    let datetime: NaiveDateTime = date.and_time(time);

    let note_type = note_type.unwrap_or(NoteTypes::Other);
    add_row(db, title, note_type, text, datetime).await;
}
