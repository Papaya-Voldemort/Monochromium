use crate::types::MonoError;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub fn write_all(path: &PathBuf, content: impl AsRef<[u8]>) -> Result<(), MonoError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut temp = NamedTempFile::new_in(path.parent().unwrap_or(std::path::Path::new(".")))?;
    temp.write_all(content.as_ref())?;
    temp.persist_noclobber(path)
        .map_err(|err| MonoError::Io(err.error))?;

    Ok(())
}
