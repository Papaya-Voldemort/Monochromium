use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::{to_string_pretty, from_str};

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



pub fn config_path() -> PathBuf {
    let project_dirs =
        ProjectDirs::from("com", "monochromium", "monochromium")
            .expect("Could not determine config directory");

    project_dirs.config_dir().join("config.toml")
}

pub fn create_config() {
    let path = config_path();

    if path.exists() {
        return;
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .expect("Could not create config directory");
    }

    let config = Config::default();

    let contents =
        to_string_pretty(&config)
            .expect("Could not serialize config");

    fs::write(path, contents)
        .expect("Could not write config");
}

pub fn load_config() -> Config {
    create_config();

    let contents =
        fs::read_to_string(config_path())
            .expect("Could not read config");

    from_str(&contents)
        .expect("Invalid config.toml")
}