use crate::types::NoteTypes;
use crate::utils::trim_text;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn make_title(
    text: String,
    note_type: NoteTypes,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
) -> String {
    match note_type {
        NoteTypes::CheckIn => {
            let now = Local::now();
            let time = time.unwrap_or(now.time());
            let date = date.unwrap_or(now.date_naive());
            let datetime: NaiveDateTime = date.and_time(time);
            let readable_date = datetime.format("%b %-d, %-I:%M %p").to_string();
            format!("Check-In: {}", readable_date)
        }
        _ => trim_text(text.clone(), 5),
    }
}
