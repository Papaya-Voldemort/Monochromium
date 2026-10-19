use crate::crypto::decrypt_with_password;
use crate::database::Database;
use crate::storage::{check_file_type, shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use crate::utils::prompt_unlock_password;
use std::fs;

pub fn unlock(db: &Database, note_id: u32) -> Result<String, MonoError> {
    let note = db.read_single_row(note_id)?;

    if check_file_type(&note.file_link)? != "enc" {
        return Ok(format!(
            "This file is already unlocked! Lock it with `mono lock {}`!",
            note.id
        ));
    }

    let password = prompt_unlock_password()?;

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

    shred_and_delete(path).map_err(|err| {
        MonoError::Crypto(format!(
            "Source cleanup failed: {err}; recovery copy: {}",
            new_path.display()
        ))
    })?;

    db.update_row(
        note_id,
        new_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    )
    .map_err(|err| {
        MonoError::Crypto(format!(
            "Database update failed: {err}; recovery copy: {}",
            new_path.display()
        ))
    })?;

    Ok("Decrypted successfully!".to_string())
}
