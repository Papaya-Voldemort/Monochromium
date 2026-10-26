use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::{MonoError, NoteTypes};
use crate::utils::{is_locked, locked_notice, pretty_notes};
use chrono::{Duration, Local, NaiveDate};

pub fn list(
    db: &Database,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    today: bool,
    since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, MonoError> {
    let final_limit: u16 = limit.unwrap_or(15);
    let current_date = Local::now().date_naive();
    let (since, until) = if today {
        (
            Some(since.map_or(current_date, |date| date.max(current_date))),
            Some(current_date + Duration::days(1)),
        )
    } else {
        (since, None)
    };

    let mut notes = db.read_rows(final_limit as i32, note_type, since, until)?;

    if view {
        for note in &mut notes {
            if is_locked(&note.file_link)? {
                note.content = Some(locked_notice(note.id)?);
                continue;
            }
            note.content = read_file_if_exists(&note.file_link)?;
        }
    }

    Ok(pretty_notes(notes, view, false))
}
