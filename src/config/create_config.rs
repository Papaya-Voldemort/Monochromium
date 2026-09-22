use crate::types::MonoError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::{from_str, to_string_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    default_editor_command: String,
    pub checkin_interval_minutes: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_editor_command: "nano".to_string(),
            checkin_interval_minutes: 120,
        }
    }
}

pub fn config_path() -> Result<PathBuf, MonoError> {
    let project_dirs = ProjectDirs::from("com", "monochromium", "monochromium").ok_or(
        MonoError::Config("Could not determine config directory".to_string()),
    )?;

    Ok(project_dirs.config_dir().join("config.toml"))
}

pub fn create_config() -> Result<(), MonoError> {
    let path = config_path()?;

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

    let contents = fs::read_to_string(config_path()?)?;

    Ok(from_str(&contents)?)
}
