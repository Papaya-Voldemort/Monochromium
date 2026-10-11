use crate::types::MonoError;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<String, MonoError> {
    Ok(std::fs::read_to_string(path)?)
}

pub fn read_file_if_exists(path: &PathBuf) -> Result<Option<String>, MonoError> {
    match read_file(path) {
        Ok(content) => Ok(Some(content)),
        Err(MonoError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Warning: note file {} is missing.", path.display());
            Ok(None)
        }
        Err(err) => Err(err),
    }
}
