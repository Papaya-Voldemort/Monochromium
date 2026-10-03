use crate::types::MonoError;
use std::fs;
use std::path::PathBuf;

pub fn write_all(path: PathBuf, content: String) -> Result<(), MonoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    Ok(())
}
