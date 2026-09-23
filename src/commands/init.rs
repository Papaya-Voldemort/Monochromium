use crate::config::setup_zshrc;
use crate::types::MonoError;

pub fn init() -> Result<String, MonoError> {
    match setup_zshrc() {
        Ok(_) => Ok("Successfully configured Monochromium reminders in ~/.zshrc!".to_string()),
        Err(e) => Err(MonoError::Config(format!(
            "Failed to configure zshrc: {}",
            e
        ))),
    }
}
