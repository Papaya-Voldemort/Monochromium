use crate::commands::definitions::NoteTypes;
use crate::database::add_row;
use crate::utils::{ make_title, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub async fn add(
    conn: libsql::Connection,
    text: Option<String>,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> String {
    let full_text = string_check(text, paste);
    if full_text == "Please provide a message when making your note!" {
        return full_text;
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());

    let title = make_title(full_text.clone());
    let datetime: NaiveDateTime = date.and_time(time);

    let note_type = note_type.unwrap_or(NoteTypes::Other);
    add_row(conn, title, note_type, full_text, datetime).await;

    "Note added successfully!".to_string()
}
