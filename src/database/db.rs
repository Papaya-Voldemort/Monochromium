use directories::ProjectDirs;
use rusqlite::Connection;
use std::fs;

use super::Database;

const SCHEMA: &str = include_str!("schema.sql");
impl Database {
    pub fn new() -> Self {
        // Get OS specific project directories
        if let Some(project_dirs) = ProjectDirs::from("com", "monochromium", "monochromium") {
            fs::create_dir_all(project_dirs.data_dir()).unwrap();
            let data_dir = project_dirs.data_dir();
            // println!("{}", data_dir.display());

            let db_path = data_dir.join("monochromium.db");
            // println!("{}", db_path.display());

            // Make database
            let conn = Connection::open(db_path).unwrap();

            conn.execute_batch(SCHEMA).unwrap();

            Self { conn }
        } else {
            panic!("Could not determine default database location");
        }
    }

    #[cfg(test)]
    pub fn new_in_memory() -> Self {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(SCHEMA).unwrap();

        Self { conn }
    }
}
