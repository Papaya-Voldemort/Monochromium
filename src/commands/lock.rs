use crate::cypto::encrypt_with_password;
use crate::database::Database;
use crate::storage::{read_file, shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use std::path::PathBuf;

pub fn lock(db: &Database, note_id: u32, password: String) -> Result<String, MonoError> {
    let note = db.read_single_row(note_id)?;

    let note_path = note.file_link;

    let content = read_file(&note_path)?;

    let encrypted = encrypt_with_password(password.as_bytes(), content.as_bytes())?;

    let bytes: &[u8] = &encrypted;

    let mut new_os_str = note_path.clone().into_os_string();
    new_os_str.push(".enc");
    let new_file_path = PathBuf::from(new_os_str);

    write_all(&new_file_path, bytes)?;
    shred_and_delete(&note_path)?;

    db.update_row(
        note_id,
        new_file_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    )?;

    Ok("Encrypted successfully!".to_string())
}
