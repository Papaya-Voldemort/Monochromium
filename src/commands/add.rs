use crate::commands::definitions::NoteTypes;
use crate::database::{add_row, read_single_row};
use crate::utils::{copy, make_title, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub async fn add(
    conn: libsql::Connection,
    text: Option<String>,
    note_type: Option<NoteTypes>,
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

    let title = make_title(full_text.clone());
    let datetime: NaiveDateTime = date.and_time(time);

    let note_type = note_type.unwrap_or(NoteTypes::Other);
    let result = add_row(conn.clone(), title, note_type, full_text, datetime).await?;
    let note = read_single_row(conn.clone(), result).await?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title, note.id, note.date, note.content
    );
    copy(output.clone());

    Ok(output)
}
