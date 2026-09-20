mod create_config;
mod zshrc;

pub use create_config::{Config, create_config, load_config};
pub use zshrc::setup_zshrc;
