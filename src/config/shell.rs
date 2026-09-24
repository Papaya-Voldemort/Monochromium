use directories::UserDirs;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

pub fn setup_zshrc() -> std::io::Result<()> {
    if let Some(user_dirs) = UserDirs::new() {
        let zshrc_path = user_dirs.home_dir().join(".zshrc");

        let existing = read_to_string(&zshrc_path).unwrap_or_default();

        if existing.contains("# >>> monochromium >>>") {
            return Ok(());
        }

        let mut zshrc = OpenOptions::new()
            .append(true)
            .create(true)
            .open(zshrc_path)?;

        let config = r#"
# >>> monochromium >>>
mono reminder 2>/dev/null
# <<< monochromium <<<
"#;

        zshrc.write_all(config.as_bytes())?;
    }

    Ok(())
}
