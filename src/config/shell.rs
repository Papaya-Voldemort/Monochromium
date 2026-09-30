use crate::types::MonoError;
use directories::UserDirs;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::PathBuf;

enum Shell {
    Bash,
    Zsh,
    Fish,
    Unknown,
}

fn current_shell() -> Result<Shell, MonoError> {
    let shell = std::env::var("SHELL").map_err(|_| MonoError::ShellNotFound)?;

    let shell_name = std::path::Path::new(&shell)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(MonoError::ShellNotFound)?;

    match shell_name {
        "fish" => Ok(Shell::Fish),
        "zsh" => Ok(Shell::Zsh),
        "bash" => Ok(Shell::Bash),
        _ => Ok(Shell::Unknown),
    }
}

pub fn setup_shell() -> Result<PathBuf, MonoError> {
    let user_dirs = UserDirs::new().ok_or(MonoError::ShellNotFound)?;

    let home = user_dirs.home_dir();

    let shell_path = match current_shell()? {
        Shell::Fish => home.join(".config/fish/config.fish"),
        Shell::Zsh => home.join(".zshrc"),
        Shell::Bash => home.join(".bashrc"),
        Shell::Unknown => return Err(MonoError::Config("unsupported shell".to_string())),
    };

    if let Some(parent) = shell_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let existing = read_to_string(&shell_path).unwrap_or_default();

    if existing.contains("# >>> monochromium >>>") {
        return Ok(shell_path);
    }

    let mut shell_config = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&shell_path)?;

    let config = r#"
# >>> monochromium >>>
mono reminder 2>/dev/null
# <<< monochromium <<<
"#;

    shell_config.write_all(config.as_bytes())?;

    Ok(shell_path)
}
