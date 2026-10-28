use crate::config::setup_shell;
use crate::types::MonoError;

#[cfg(not(windows))]
pub fn init() -> Result<String, MonoError> {
    match setup_shell() {
        Ok(shell_path) => Ok(format!(
            "Successfully configured Monochromium reminders in {}",
            shell_path.display()
        )),
        Err(e) => Err(MonoError::Config(format!(
            "Failed to configure your shell: {}",
            e
        ))),
    }
}

#[cfg(windows)]
pub fn init() -> Result<String, MonoError> {
    Err(MonoError::Config(
        "mono init is not supported on Windows yet — reminders require a POSIX shell. \
         See https://github.com/Papaya-Voldemort/Monochromium/issues/32 to request support."
            .to_string(),
    ))
}
