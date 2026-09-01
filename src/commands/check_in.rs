use crate::commands::definitions::NoteTypes;
use crate::database::add_row;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use crate::utils::{make_title, string_check};

pub async fn check_in(
    conn: libsql::Connection,
    text: Option<String>,
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

    let note_type = NoteTypes::CheckIn;
    add_row(conn, title, note_type, full_text, datetime).await;

    "Check in added successfully!".to_string()
}
