use crate::cypto::decrypt_with_password;
use crate::database::Database;
use crate::storage::{shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use std::fs;

pub fn unlock(db: &Database, note_id: u32, password: String) -> Result<String, MonoError> {
    let note = db.read_single_row(note_id)?;
    let path = &note.file_link;

    let encrypted_payload = fs::read(path)?;

    let decrypted_bytes = decrypt_with_password(password.as_bytes(), &encrypted_payload)?;

    let decrypted_string = String::from_utf8(decrypted_bytes)
        .map_err(|err| MonoError::Crypto(format!("Invalid UTF-8 payload: {err}")))?;

    let new_path = &note.file_link.with_extension("");

    write_all(new_path, decrypted_string)?;
    shred_and_delete(path)?;

    db.update_row(
        note_id,
        new_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    )?;

    Ok("Decrypted successfully!".to_string())
}
