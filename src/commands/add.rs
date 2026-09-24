use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use crate::utils::{make_title, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn add(
    db: &Database,
    text: Option<String>,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> Result<String, MonoError> {
    let full_text = string_check(text, paste)?;
    if full_text == "Please provide a message when making your note!" {
        return Err(MonoError::InvalidInput(full_text));
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());
    let note_type = note_type.unwrap_or(NoteTypes::Other);

    let title = make_title(full_text.clone(), note_type.clone(), Some(time), Some(date));
    let datetime: NaiveDateTime = date.and_time(time);

    let result = db.add_row(title, note_type, full_text, datetime)?;
    let note = db.read_single_row(result)?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title,
        note.id,
        note.date.format("%b %d, %Y at%l:%M %p"),
        note.content
    );

    Ok(output)
}
