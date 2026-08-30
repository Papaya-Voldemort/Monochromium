use directories::ProjectDirs;
use std::fs;
use std::fs::OpenOptions;

struct Config {
    default_editor_command: String,
}

pub fn create_config() {
    if let Some(project_dirs) = ProjectDirs::from("com", "monochrome", "monochrome") {
        fs::create_dir_all(project_dirs.config_dir()).unwrap();
        let config_dir = project_dirs.config_dir();
        println!("{}", config_dir.display());

    let mut config_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_dir.join("config.toml"));
    }
}
