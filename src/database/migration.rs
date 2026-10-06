use crate::config::load_config;
use crate::database::Database;
use crate::database::db::SCHEMA;
use crate::storage::write_all;
use crate::types::{MonoError, OldNote};
use crate::utils::slugify;
use chrono::NaiveDateTime;
use rusqlite::{Connection, Row, params};

const CURRENT_SCHEMA_VERSION: u32 = 2;

impl Database {
    pub fn check_version(conn: &mut Connection) -> Result<(), MonoError> {
        let current_version: u32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

        if current_version == 0 {
            conn.execute_batch(SCHEMA)?;
            conn.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;
        } else if current_version < CURRENT_SCHEMA_VERSION {
            Self::run_migrations(conn, current_version)?;
        }

        Ok(())
    }

    fn run_migrations(conn: &mut Connection, mut version: u32) -> Result<(), MonoError> {
        if version == 1 {
            Self::migrate_v1_to_v2(conn)?;
            version = 2;
        }

        let tx = conn.transaction()?;

        tx.pragma_update(None, "user_version", version)?;
        tx.commit()?;

        Ok(())
    }

    fn migrate_v1_to_v2(conn: &mut Connection) -> Result<(), MonoError> {
        let mut stmt = conn.prepare("SELECT id, title, type, date, content FROM notes")?;
        let notes = stmt
            .query_map(params![], parse_old_note)?
            .collect::<Result<Vec<OldNote>, _>>()?;

        drop(stmt);

        let path = load_config()?.note_location;

        let mut migrated_files = Vec::new();

        for note in notes {
            let slug = slugify(&note.title);
            let id = note.id;
            let filename = format!("{id:04}_{slug}.md");
            let full_path = path.join(filename);
            write_all(&full_path, note.content)?;

            migrated_files.push((note.id, full_path));
        }

        let tx = conn.transaction()?;

        tx.execute_batch("ALTER TABLE notes ADD COLUMN file_link TEXT")?;

        for (id, path) in migrated_files {
            tx.execute(
                "UPDATE notes SET file_link = ?1 WHERE id = ?2",
                params![path.to_string_lossy(), id],
            )?;
        }

        tx.execute_batch("ALTER TABLE notes DROP COLUMN content;")?;

        tx.commit()?;

        Ok(())
    }
}

fn parse_old_note(row: &Row) -> Result<OldNote, rusqlite::Error> {
    let date_str: String = row.get(3)?;

    let date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f").map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(err))
    })?;

    Ok(OldNote {
        id: row.get(0)?,
        title: row.get(1)?,
        note_type: row.get(2)?,
        date,
        content: row.get(4)?,
    })
}
