use crate::cypto::encrypt_with_password;
use crate::database::Database;
use crate::storage::{read_file, shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use crate::utils::prompt_lock_password;
use std::fs;
use std::path::PathBuf;

pub fn lock(db: &Database, note_id: u32) -> Result<String, MonoError> {
    let password = prompt_lock_password()?;

    let note = db.read_single_row(note_id)?;
    let note_path = note.file_link;

    let content = read_file(&note_path)?;
    let encrypted = encrypt_with_password(password.as_bytes(), content.as_bytes())?;

    let mut new_os_str = note_path.clone().into_os_string();
    new_os_str.push(".enc");
    let new_file_path = PathBuf::from(new_os_str);

    if new_file_path.exists() {
        return Err(MonoError::Crypto(
            "Encrypted destination already exists".into(),
        ));
    }

    write_all(&new_file_path, &encrypted)?;

    if let Err(db_error) = db.update_row(
        note_id,
        new_file_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    ) {
        if let Err(cleanup_error) = fs::remove_file(&new_file_path) {
            return Err(MonoError::Crypto(format!(
                "Database update failed: {db_error}; \
                 cleanup also failed: {cleanup_error}"
            )));
        }

        return Err(db_error.into());
    }

    shred_and_delete(&note_path)?;

    Ok("Encrypted successfully!".to_string())
}
