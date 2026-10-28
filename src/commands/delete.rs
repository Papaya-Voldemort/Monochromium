use crate::database::Database;
use crate::storage::delete_file;
use crate::types::MonoError;
use std::fs;
use std::path::Path;
use tempfile::Builder;

pub fn delete(db: &Database, note_id: u32, approve: bool) -> Result<String, MonoError> {
    if approve {
        let Some(note) = db.read_single_row_opt(note_id)? else {
            return Ok(format!("No note found with ID {note_id}"));
        };

        let tx = db.conn.unchecked_transaction()?;
        db.delete_note(note_id)?;

        let backup = if note.file_link.try_exists()? {
            let backup = Builder::new()
                .prefix(".mono-delete-")
                .tempfile_in(note.file_link.parent().unwrap_or(Path::new(".")))?
                .into_temp_path()
                .keep()
                .map_err(|err| MonoError::Io(err.error))?;
            if let Err(err) = fs::rename(&note.file_link, &backup) {
                delete_file(backup)?;
                return Err(err.into());
            }
            Some(backup)
        } else {
            None
        };

        if let Err(err) = tx.commit() {
            if let Some(backup) = backup
                && let Err(restore_err) = fs::rename(&backup, &note.file_link)
            {
                return Err(MonoError::Io(std::io::Error::new(
                    restore_err.kind(),
                    format!(
                        "{restore_err}; note contents preserved in {}",
                        backup.display()
                    ),
                )));
            }
            return Err(err.into());
        }
        if let Some(backup) = backup {
            delete_file(backup)?;
        }
        return Ok(format!("Deleted note with ID of {}", note_id));
    }
    Ok(format!(
        "Please pass the '--approve' to confirm the deletion of note {}",
        note_id
    ))
}
