use crate::config::load_config;
use crate::types::MonoError;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::Builder;

fn run_editor_on(path: &Path) -> Result<(), MonoError> {
    let editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .or_else(|_| load_config().map(|c| c.default_editor_command))?;

    let status = editor_command(&editor)?
        .arg(path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                MonoError::Config(format!(
                    "could not launch editor '{editor}' — set $EDITOR or $VISUAL"
                ))
            } else {
                MonoError::Io(err)
            }
        })?;

    if !status.success() {
        return Err(MonoError::Io(std::io::Error::other(format!(
            "Editor exited unsuccessfully: {status}"
        ))));
    }
    Ok(())
}

pub fn edit_text(initial: &str) -> Result<Option<String>, MonoError> {
    let mut temp = Builder::new()
        .prefix("cli-add-")
        .suffix(".md")
        .tempfile_in(std::env::temp_dir())?;
    temp.write_all(initial.as_bytes())?;
    temp.flush()?;

    run_editor_on(temp.path())?;

    let edited = fs::read_to_string(temp.path())?;
    if edited == initial || edited.trim().is_empty() {
        return Ok(None);
    }
    Ok(Some(edited))
}

pub fn edit_file(target_path: &PathBuf) -> Result<bool, MonoError> {
    let original_content: String = if target_path.exists() {
        fs::read_to_string(target_path)?
    } else {
        String::new()
    };

    let mut temp = Builder::new()
        .prefix("cli-edit-")
        .suffix(".md")
        .tempfile_in(target_path.parent().unwrap_or(Path::new(".")))?;
    temp.write_all(original_content.as_bytes())?;
    temp.flush()?;

    run_editor_on(temp.path())?;

    let edited_content = fs::read_to_string(temp.path())?;
    if edited_content == original_content || edited_content.trim().is_empty() {
        return Ok(false);
    }

    temp.persist(target_path)
        .map_err(|err| MonoError::Io(err.error))?;

    Ok(true)
}

fn editor_command(editor: &str) -> Result<Command, MonoError> {
    let parts = if Path::new(editor).is_file() {
        vec![editor.to_string()]
    } else {
        shlex::split(editor)
            .ok_or_else(|| MonoError::Config("Invalid quoting in editor command".to_string()))?
    };
    let (program, args) = parts
        .split_first()
        .filter(|(program, _)| !program.is_empty())
        .ok_or_else(|| MonoError::Config("Editor command cannot be empty".to_string()))?;
    let mut command = Command::new(program);
    command.args(args);
    Ok(command)
}

pub fn _overwrite(path: &PathBuf, ciphertext: &[u8]) -> Result<(), MonoError> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write_all(ciphertext)?;
    file.sync_all()?;
    Ok(())
}

pub fn _rename(path: &PathBuf, new_path: &PathBuf) -> Result<(), MonoError> {
    fs::rename(path, new_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_editor_arguments_and_quoted_paths() -> Result<(), MonoError> {
        let command = editor_command("code --wait --reuse-window")?;
        assert_eq!(command.get_program(), "code");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            ["--wait", "--reuse-window"]
        );
        let command = editor_command(r#""C:\Program Files\Editor\editor.exe" --wait"#)?;
        assert_eq!(command.get_program(), r"C:\Program Files\Editor\editor.exe");
        assert_eq!(command.get_args().collect::<Vec<_>>(), ["--wait"]);
        assert!(editor_command(" ").is_err());
        assert!(editor_command("editor 'unfinished").is_err());
        Ok(())
    }

    #[test]
    fn edit_text_returns_saved_buffer() -> Result<(), MonoError> {
        unsafe {
            std::env::set_var("VISUAL", r#"sh -c 'echo hello from editor > "$0"'"#);
        }
        let text = edit_text("")?.expect("editor produced text");
        assert_eq!(text.trim_end(), "hello from editor");
        Ok(())
    }
}
