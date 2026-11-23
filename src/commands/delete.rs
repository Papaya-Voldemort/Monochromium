use crate::database::Database;
use crate::storage::delete_files;
use crate::types::MonoError;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::Builder;

/// Moves `path` aside to a hidden backup file so the deletion can be undone
/// if the database commit fails. Returns `None` if the file doesn't exist.
fn backup_file(path: &Path) -> Result<Option<PathBuf>, MonoError> {
    if !path.try_exists()? {
        return Ok(None);
    }

    let backup = Builder::new()
        .prefix(".mono-delete-")
        .tempfile_in(path.parent().unwrap_or(Path::new(".")))?
        .into_temp_path()
        .keep()
        .map_err(|err| MonoError::Io(err.error))?;

    if let Err(err) = fs::rename(path, &backup) {
        delete_files(vec![&backup])?;
        return Err(err.into());
    }

    Ok(Some(backup))
}

/// Moves every backup back to its original location, attempting all of them
/// and reporting the first failure.
fn restore_backups(backups: &[(&Path, PathBuf)]) -> Result<(), MonoError> {
    let mut first_err = None;

    for (original, backup) in backups {
        if let Err(err) = fs::rename(backup, original) {
            first_err.get_or_insert_with(|| {
                MonoError::Io(std::io::Error::new(
                    err.kind(),
                    format!("{err}; note contents preserved in {}", backup.display()),
                ))
            });
        }
    }

    first_err.map_or(Ok(()), Err)
}

pub fn delete(db: &Database, note_ids: Vec<u32>, approve: bool) -> Result<Vec<String>, MonoError> {
    if !approve {
        let ids = note_ids
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        return Ok(vec![format!(
            "Please pass the '--approve' to confirm the deletion of note(s) {ids}"
        )]);
    }

    let mut output: Vec<String> = Vec::new();

    let notes = db.read_specific_rows(&note_ids)?;
    let found_ids: HashSet<u32> = notes.iter().map(|note| note.id).collect();

    let mut reported = HashSet::new();
    for id in note_ids
        .iter()
        .filter(|id| !found_ids.contains(id) && reported.insert(**id))
    {
        output.push(format!("Note {id} does not exist"));
    }

    if notes.is_empty() {
        return Ok(output);
    }

    // Dropping `tx` without committing rolls the database back.
    let tx = db.conn.unchecked_transaction()?;
    let ids_to_delete: Vec<u32> = notes.iter().map(|note| note.id).collect();
    db.delete_note(&ids_to_delete)?;

    let mut backups: Vec<(&Path, PathBuf)> = Vec::new();
    for note in &notes {
        match backup_file(&note.file_link) {
            Ok(Some(backup)) => backups.push((note.file_link.as_path(), backup)),
            Ok(None) => {}
            Err(err) => {
                restore_backups(&backups)?;
                return Err(err);
            }
        }
    }

    if let Err(err) = tx.commit() {
        restore_backups(&backups)?;
        return Err(err.into());
    }

    let backup_paths: Vec<&Path> = backups.iter().map(|(_, backup)| backup.as_path()).collect();
    delete_files(backup_paths)?;

    for note in &notes {
        output.push(format!("Deleted note with ID of {}", note.id));
    }

    Ok(output)
}
