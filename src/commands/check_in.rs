use crate::database::Database;
use crate::types::NoteTypes;
use crate::utils::string_check;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn check_in(
    db: &Database,
    text: Option<String>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let full_text = string_check(text, paste);
    if full_text == "Please provide a message when making your note!" {
        return Ok(full_text);
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());

    let datetime: NaiveDateTime = date.and_time(time);
    let readable_date = datetime.format("%b %-d, %-I:%M %p").to_string();
    let title = format!("Check-In: {}", readable_date);

    let datetime: NaiveDateTime = date.and_time(time);

    let note_type = NoteTypes::CheckIn;
    let _output = db.add_row(title, note_type, full_text, datetime)?;

    Ok("check in added successfully!".to_string())
}
