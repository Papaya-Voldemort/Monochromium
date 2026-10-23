use crate::config::create_config;
use crate::storage::edit_file;
use crate::types::MonoError;
use crate::utils::{PathType, get_path};

pub fn config() -> Result<String, MonoError> {
    create_config()?;
    let config_path = get_path(PathType::Config)?.join("config.toml");
    let saved = edit_file(&config_path)?;

    if saved {
        Ok("Edit made successfully!".to_string())
    } else {
        Ok("Config file opened successfully".to_string())
    }
}
