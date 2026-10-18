use crate::types::MonoError;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn delete_file(path: PathBuf) -> Result<(), MonoError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(MonoError::Io(err)),
    }
}

pub fn shred_and_delete(path: &PathBuf) -> Result<(), MonoError> {
    let mut file = OpenOptions::new().write(true).open(path)?;

    let metadata = file.metadata()?;
    let length = metadata.len() as usize;

    let zeros = vec![0u8; length];
    file.write_all(&zeros)?;
    file.flush()?;

    file.sync_all()?;

    std::fs::remove_file(path)?;

    Ok(())
}
