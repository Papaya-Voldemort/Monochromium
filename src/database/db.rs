use directories::ProjectDirs;
use std::fs;
use libsql::Builder;

const SCHEMA: &str = include_str!("schema.sql");

pub async fn make_db() {
    // Get OS specific project directories
    if let Some(project_dirs) = ProjectDirs::from("com", "monochrome", "monochrome") {
        fs::create_dir_all(project_dirs.config_dir()).unwrap();
        let config_dir = project_dirs.config_dir();
        println!("{}", config_dir.display());

        fs::create_dir_all(project_dirs.cache_dir()).unwrap();
        let cache_dir = project_dirs.cache_dir();
        println!("{}", cache_dir.display());

        fs::create_dir_all(project_dirs.data_dir()).unwrap();
        let data_dir = project_dirs.data_dir();
        println!("{}", data_dir.display());

        let db_path = data_dir.join("monochrome.db");
        println!("{}", db_path.display());

        // Make database
        let db = Builder::new_local(db_path).build().await.unwrap();

        let conn = db.connect().unwrap();

        conn.execute_batch(SCHEMA).await.unwrap();
    } else {
        println!("Could not determine default database location");
    }
}
