use crate::storage::check_file_type;
use crate::types::MonoError;
use std::path::Path;

pub fn is_locked(path: &Path) -> Result<bool, MonoError> {
    let file_type = check_file_type(path)?;
    Ok(file_type.eq_ignore_ascii_case("enc"))
}

pub fn locked_notice(id: u32) -> Result<String, MonoError> {
    Ok(format!(
        "This file is currently locked! Unlock it with `mono unlock {}`!",
        id
    ))
}
