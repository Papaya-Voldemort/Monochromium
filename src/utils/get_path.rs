use crate::types::MonoError;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub enum PathType {
    Database,
    Config,
}

pub fn get_path(path_type: PathType) -> Result<PathBuf, MonoError> {
    let project_dirs = ProjectDirs::from("com", "monochromium", "monochromium")
        .ok_or(MonoError::DirectoriesNotFound)?;

    let target_dir = match path_type {
        PathType::Database => project_dirs.data_dir(),
        PathType::Config => project_dirs.config_dir(),
    };

    fs::create_dir_all(target_dir)?;

    Ok(target_dir.to_path_buf())
}
