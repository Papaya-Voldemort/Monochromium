use crate::database::Database;
use crate::storage::read_file;
use crate::types::MonoError;
use crate::utils::{DATETIME_FORMAT, is_locked, locked_notice};

pub fn view(db: &Database, note_id: u32, no_format: bool) -> Result<String, MonoError> {
    let note = db.read_single_row(note_id)?;

    if is_locked(&note.file_link)? {
        return locked_notice(note.id);
    }

    if no_format {
        return read_file(&note.file_link);
    }

    let content = read_file(&note.file_link)?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title,
        note.id,
        note.date.format(DATETIME_FORMAT),
        content
    );

    Ok(output)
}
