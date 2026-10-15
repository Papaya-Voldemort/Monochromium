use crate::types::MonoError;
use std::ffi::OsStr;
use std::path::Path;

pub fn check_file_type(path: &Path) -> Result<String, MonoError> {
    let file = Path::new(path);

    if let Some(ext) = file.extension().and_then(OsStr::to_str) {
        return Ok(ext.to_string());
    }

    Err(MonoError::FileExtension)
}
