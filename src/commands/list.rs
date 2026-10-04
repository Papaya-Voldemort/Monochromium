use crate::database::Database;
use crate::storage::read_file;
use crate::types::{MonoError, NoteTypes};
use crate::utils::pretty_notes;
use chrono::NaiveDate;
use std::path::PathBuf;

pub fn list(
    db: &Database,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    _today: bool,
    _since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, MonoError> {
    let final_limit: u16 = limit.unwrap_or(15);
    let mut notes = db.read_rows(final_limit as i32, note_type)?;

    for note in &mut notes {
        note.content = Some(read_file(&PathBuf::from(&note.file_link))?);
    }

    Ok(pretty_notes(notes, view))
}
