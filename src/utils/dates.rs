use crate::types::Note;
use chrono::NaiveDate;

pub fn day_of(note: &Note) -> NaiveDate {
    note.date.date()
}
