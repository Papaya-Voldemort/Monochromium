use crate::types::MonoError;
use crate::utils::{PathType, get_path};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::{from_str, to_string_pretty};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    default_editor_command: String,
    pub checkin_interval_minutes: i64,
    pub show_todo_list: bool,
    pub note_location: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let default_notes = UserDirs::new()
            .and_then(|u| u.document_dir().map(|d| d.join("Monochromium")))
            .unwrap_or_else(|| PathBuf::from("Monochromium"));

        Self {
            default_editor_command: "nano".to_string(),
            checkin_interval_minutes: 120,
            show_todo_list: true,
            note_location: default_notes,
        }
    }
}

pub fn create_config() -> Result<(), MonoError> {
    let path = get_path(PathType::Config)?.join("config.toml");

    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config = Config::default();
    let contents = to_string_pretty(&config)?;

    fs::write(path, contents)?;

    Ok(())
}

pub fn load_config() -> Result<Config, MonoError> {
    create_config()?;

    let contents = fs::read_to_string(get_path(PathType::Config)?.join("config.toml"))?;

    Ok(from_str(&contents)?)
}
