mod create_config;
mod shell;

pub use create_config::{Config, load_config};
pub use shell::setup_zshrc;
