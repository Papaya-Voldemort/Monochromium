use crate::commands::definitions::NoteTypes;
use crate::database::add_row;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};


pub async fn check_in(
    db: libsql::Database,
    text: String,
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

    let note_type = NoteTypes::CheckIn;
    add_row(db, title, note_type, text, datetime).await;
}
