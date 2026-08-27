use crate::commands::definitions::NoteTypes;
use chrono::{NaiveDate, NaiveTime};

pub fn add(
    text: String,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) {
}
