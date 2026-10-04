use crate::types::MonoError;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<String, MonoError> {
    Ok(std::fs::read_to_string(path)?)
}
