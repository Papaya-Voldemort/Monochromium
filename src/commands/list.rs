use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::{MonoError, NoteTypes};
use crate::utils::{is_locked, locked_notice, pretty_notes};
use chrono::{Local, NaiveDate};

pub fn list(
    db: &Database,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    today: bool,
    since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, MonoError> {
    let final_limit: u16 = limit.unwrap_or(15);
    let read_limit = if today || since.is_some() {
        -1
    } else {
        final_limit as i32
    };
    let mut notes = db.read_rows(read_limit, note_type)?;
    let current_date = Local::now().date_naive();
    notes.retain(|note| {
        (!today || note.date.date() == current_date)
            && since.is_none_or(|since| note.date.date() >= since)
    });
    notes.truncate(final_limit as usize);

    if view {
        for note in &mut notes {
            if is_locked(&note.file_link)? {
                note.content = Some(locked_notice(note.id)?);
                continue;
            }
            note.content = read_file_if_exists(&note.file_link)?;
        }
    }

    Ok(pretty_notes(notes, view))
}
