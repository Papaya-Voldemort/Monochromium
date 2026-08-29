use directories::ProjectDirs;
use libsql::{Builder, Database};
use std::fs;

const SCHEMA: &str = include_str!("schema.sql");

pub async fn make_db() -> libsql::Database {
    // Get OS specific project directories
    if let Some(project_dirs) = ProjectDirs::from("com", "monochrome", "monochrome") {
        fs::create_dir_all(project_dirs.data_dir()).unwrap();
        let data_dir = project_dirs.data_dir();
        // println!("{}", data_dir.display());

        let db_path = data_dir.join("monochromium.db");
        // println!("{}", db_path.display());

        // Make database
        let db = Builder::new_local(db_path).build().await.unwrap();

        let conn = db.connect().unwrap();

        conn.execute_batch(SCHEMA).await.unwrap();

        db
    } else {
        panic!("Could not determine default database location");
    }
}
