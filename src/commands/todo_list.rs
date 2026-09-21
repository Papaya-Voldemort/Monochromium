use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use crate::utils::pretty_notes;
use chrono::NaiveDate;

pub fn list_todos(
    db: &Database,
    limit: Option<u16>,
    _today: bool,
    _since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, MonoError> {
    let limit = limit.unwrap_or(10);
    let notes = db.read_rows(limit as i32, Some(NoteTypes::Todo))?;
    Ok(pretty_notes(notes, view))
}
