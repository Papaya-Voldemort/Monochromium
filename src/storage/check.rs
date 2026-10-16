use crate::types::MonoError;
use std::ffi::OsStr;
use std::path::Path;

pub fn check_file_type(path: &Path) -> Result<String, MonoError> {
    let file = Path::new(path);
    if file.extension().is_none() {
        return Ok(String::new());
    }

    if let Some(ext) = file.extension().and_then(OsStr::to_str) {
        return Ok(ext.to_string());
    }

    Err(MonoError::FileExtension)
}

#[test]
fn accepts_extensionless_files() {
    assert_eq!(check_file_type(Path::new("note")).unwrap(), "");
}
