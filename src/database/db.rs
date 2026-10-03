use super::Database;
use crate::types::MonoError;
use crate::utils::{PathType, get_path};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

pub const SCHEMA: &str = include_str!("schema.sql");
impl Database {
    pub fn new() -> Result<Self, MonoError> {
        let data_dir = get_path(PathType::Database)?;
        let db_path = data_dir.join("monochromium.db");
        let conn = Connection::open(db_path)?;

        conn.execute_batch(SCHEMA)?;

        Ok(Self { conn })
    }

    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, MonoError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }
}
