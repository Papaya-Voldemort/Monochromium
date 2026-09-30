use crate::config::setup_shell;
use crate::types::MonoError;

pub fn init() -> Result<String, MonoError> {
    match setup_shell() {
        Ok(shell_path) => Ok(format!(
            "Successfully configured Monochromium reminders in {}",
            shell_path.display()
        )),
        Err(e) => Err(MonoError::Config(format!(
            "Failed to configure zshrc: {}",
            e
        ))),
    }
}
