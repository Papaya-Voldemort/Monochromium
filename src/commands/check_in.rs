use crate::commands::definitions::NoteTypes;
use crate::database::add_row;
use crate::utils::{make_title, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub async fn check_in(
    conn: libsql::Connection,
    text: Option<String>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let full_text = string_check(text, paste);
    if full_text == "Please provide a message when making your note!" {
        println!("returned");
        return Ok(full_text);
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());

    let datetime: NaiveDateTime = date.and_time(time);
    let readable_date = datetime.format("%b %-d, %-I:%M %p").to_string();
    let title = format!("Check-In: {}", readable_date);
    println!("{}", title);

    let datetime: NaiveDateTime = date.and_time(time);

    let note_type = NoteTypes::CheckIn;
    let output = add_row(conn, title, note_type, full_text, datetime).await?;

    Ok("Check in added successfully!".to_string())
}
