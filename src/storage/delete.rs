use crate::types::MonoError;
use std::fs;
use std::path::PathBuf;

pub fn delete_file(path: PathBuf) -> Result<(), MonoError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(MonoError::Io(err)),
    }
}
