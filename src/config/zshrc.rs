use directories::UserDirs;
use tokio::fs::{read_to_string, OpenOptions};
use tokio::io::AsyncWriteExt;

pub async fn setup_zshrc() -> std::io::Result<()> {
    if let Some(user_dirs) = UserDirs::new() {
        let zshrc_path = user_dirs.home_dir().join(".zshrc");

        let existing = read_to_string(&zshrc_path).await.unwrap_or_default();

        if existing.contains("# >>> monochromium >>>") {
            println!("zshrc already setup");
            return Ok(());
        }

        let mut zshrc = OpenOptions::new()
            .append(true)
            .create(true)
            .open(zshrc_path)
            .await?;

        let config = "\n# >>> monochromium >>>\n\
    _mono_checkin_reminder() {\n\
        mono reminder 2>/dev/null\n\
    }\n\
    precmd_functions+=(_mono_checkin_reminder)\n\
    # <<< monochromium <<<\n";

        zshrc.write_all(config.as_bytes()).await?;
    }

    Ok(())
}
