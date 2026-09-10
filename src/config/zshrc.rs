use directories::UserDirs;
use tokio::fs::{OpenOptions, read_to_string};
use tokio::io::AsyncWriteExt;

pub async fn setup_zshrc() -> std::io::Result<()> {
    if let Some(user_dirs) = UserDirs::new() {
        let zshrc_path = user_dirs.home_dir().join(".zshrc");

        let existing = read_to_string(&zshrc_path).await.unwrap_or_default();

        if existing.contains("# >>> monochromium >>>") {
            return Ok(());
        }

        let mut zshrc = OpenOptions::new()
            .append(true)
            .create(true)
            .open(zshrc_path)
            .await?;

        let config = r#"
# >>> monochromium >>>
mono reminder 2>/dev/null
# <<< monochromium <<<
"#;

        zshrc.write_all(config.as_bytes()).await?;
    }

    Ok(())
}
