use super::Database;
use crate::types::MonoError;
use crate::utils::{PathType, get_path};
use rusqlite::Connection;

pub const SCHEMA: &str = include_str!("schema.sql");
impl Database {
    pub fn new() -> Result<Self, MonoError> {
        let data_dir = get_path(PathType::Database)?;
        let db_path = data_dir.join("monochromium.db");
        let mut conn = Connection::open(db_path)?;

        conn.execute_batch(SCHEMA)?;
        Self::check_version(&mut conn)?;

        Ok(Self { conn })
    }

    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, MonoError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }
}
