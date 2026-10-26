use crate::database::Database;
use crate::storage::read_file;
use crate::types::MonoError;
use crate::utils::{is_locked, locked_notice, pretty_note};

pub fn view(
    db: &Database,
    note_id: u32,
    no_format: bool,
    verbose: bool,
) -> Result<String, MonoError> {
    let mut note = db.read_single_row(note_id)?;

    if is_locked(&note.file_link)? {
        return locked_notice(note.id);
    }

    if no_format {
        return read_file(&note.file_link);
    }

    note.content = Some(read_file(&note.file_link)?);

    let output = pretty_note(note, true, verbose);

    Ok(output)
}
