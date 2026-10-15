use crate::cypto::decrypt_with_password;
use crate::database::Database;
use crate::storage::{shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use crate::utils::prompt_unlock_password;
use std::fs;

pub fn unlock(db: &Database, note_id: u32) -> Result<String, MonoError> {
    let password = prompt_unlock_password()?;

    let note = db.read_single_row(note_id)?;
    let path = &note.file_link;

    let encrypted_payload = fs::read(path)?;

    let decrypted_bytes = decrypt_with_password(password.as_bytes(), &encrypted_payload)?;

    let decrypted_string = String::from_utf8(decrypted_bytes)
        .map_err(|err| MonoError::Crypto(format!("Invalid UTF-8 payload: {err}")))?;

    let new_path = &note.file_link.with_extension("");

    if new_path.exists() {
        return Err(MonoError::Crypto(
            "Decrypted destination already exists".into(),
        ));
    }

    write_all(new_path, decrypted_string)?;

    if let Err(db_error) = db.update_row(
        note_id,
        new_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    ) {
        // Clean up plaintext if the DB update fails.
        if let Err(cleanup_error) = shred_and_delete(new_path) {
            return Err(MonoError::Crypto(format!(
                "Database update failed: {db_error}; \
                 plaintext cleanup also failed: {cleanup_error}"
            )));
        }

        return Err(db_error.into());
    }

    shred_and_delete(path)?;

    Ok("Decrypted successfully!".to_string())
}
