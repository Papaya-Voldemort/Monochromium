use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use crate::utils::pretty_notes;
use chrono::NaiveDate;

pub fn list(
    db: &Database,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    _today: bool,
    _since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, MonoError> {
    let final_limit: u16 = limit.unwrap_or(15);
    let notes = db.read_rows(final_limit as i32, note_type)?;

    Ok(pretty_notes(notes, view))
}
