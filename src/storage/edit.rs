use crate::config::load_config;
use crate::types::MonoError;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::Builder;

pub fn edit_file(target_path: &PathBuf) -> Result<(), MonoError> {
    let original_content: String = if target_path.exists() {
        fs::read_to_string(target_path)?
    } else {
        String::new()
    };

    let mut temp = Builder::new()
        .prefix("cli-edit-")
        .suffix(".md")
        .tempfile()?;

    temp.write_all(original_content.as_bytes())?;
    temp.flush()?;

    let config = load_config()?;
    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or(config.default_editor_command);

    let status = Command::new(&editor)
        .arg(temp.path())
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(MonoError::Io(std::io::Error::other(format!(
            "Editor exited unsuccessfully: {status}"
        ))));
    }

    let edited_content = fs::read_to_string(temp.path())?;

    if edited_content == original_content {
        println!("No changes detected.");
        return Ok(());
    }

    if edited_content.trim().is_empty() {
        println!("Buffer was emptied; aborting save.");
        return Ok(());
    }

    fs::write(target_path, &edited_content)?;
    println!("File contents saved.");

    Ok(())
}
