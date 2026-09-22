use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonoError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("config serialization error: {0}")]
    ConfigSerialize(#[from] toml::ser::Error),
}
