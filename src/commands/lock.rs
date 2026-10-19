use crate::crypto::encrypt_with_password;
use crate::database::Database;
use crate::storage::{check_file_type, read_file, shred_and_delete, write_all};
use crate::types::{EditMode, MonoError};
use crate::utils::prompt_lock_password;
use std::path::PathBuf;
use zeroize::Zeroizing;

pub fn lock(db: &Database, note_id: u32) -> Result<String, MonoError> {
    let note = db.read_single_row(note_id)?;

    if check_file_type(&note.file_link)? == "enc" {
        return Ok(format!(
            "This file appears already locked! Unlock it with `mono unlock {}`!",
            note.id
        ));
    }

    let password = prompt_lock_password()?;

    let note_path = note.file_link;

    let content = Zeroizing::new(read_file(&note_path)?);
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

    shred_and_delete(&note_path).map_err(|err| {
        MonoError::Crypto(format!(
            "Source cleanup failed: {err}; recovery copy: {}",
            new_file_path.display()
        ))
    })?;

    db.update_row(
        note_id,
        new_file_path.to_string_lossy().to_string(),
        EditMode::FilePath,
    )
    .map_err(|err| {
        MonoError::Crypto(format!(
            "Database update failed: {err}; recovery copy: {}",
            new_file_path.display()
        ))
    })?;

    Ok("Encrypted successfully!".to_string())
}
