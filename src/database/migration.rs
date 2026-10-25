use crate::config::load_config;
use crate::database::Database;
use crate::database::db::SCHEMA;
use crate::storage::write_all;
use crate::types::{MonoError, OldNote};
use crate::utils::slugify;
use chrono::NaiveDateTime;
use rusqlite::{Connection, Row, params};
use std::path::Path;

const CURRENT_SCHEMA_VERSION: u32 = 3;
const V2_SCHEMA_VERSION: u32 = 2;

impl Database {
    pub fn check_version(conn: &mut Connection) -> Result<(), MonoError> {
        Self::check_version_at(conn, None)
    }

    fn check_version_at(conn: &mut Connection, directory: Option<&Path>) -> Result<(), MonoError> {
        let version: u32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

        let table_exists: bool = conn.query_row(
            "SELECT EXISTS(
            SELECT 1 FROM sqlite_master
            WHERE type = 'table' AND name = 'notes'
        )",
            [],
            |row| row.get(0),
        )?;

        if !table_exists {
            if version != 0 {
                return Err(MonoError::from(rusqlite::Error::InvalidQuery));
            }

            let tx = conn.transaction()?;
            tx.execute_batch(SCHEMA)?;
            tx.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;
            tx.commit()?;
            return Ok(());
        }

        let (has_content, has_file_link, has_value) = {
            let mut stmt = conn.prepare("PRAGMA table_info(notes)")?;

            let columns = stmt
                .query_map([], |row| row.get::<_, String>(1))?
                .collect::<Result<Vec<_>, _>>()?;

            (
                columns.iter().any(|c| c == "content"),
                columns.iter().any(|c| c == "file_link"),
                columns.iter().any(|c| c == "value"),
            )
        };

        match (has_content, has_file_link, has_value) {
            (true, false, _) if version <= CURRENT_SCHEMA_VERSION => {
                let directory = match directory {
                    Some(path) => path.to_path_buf(),
                    None => load_config()?.note_location,
                };
                Self::migrate_v1_to_v2(conn, &directory)?;
                Self::migrate_v2_to_v3(conn)?;
            }

            (false, true, false) if version <= CURRENT_SCHEMA_VERSION => {
                Self::migrate_v2_to_v3(conn)?;
            }

            (false, true, true) if version == CURRENT_SCHEMA_VERSION => {}

            (false, true, true) if version < CURRENT_SCHEMA_VERSION => {
                let tx = conn.transaction()?;
                tx.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;
                tx.commit()?;
            }

            _ => return Err(rusqlite::Error::InvalidQuery.into()),
        }

        Ok(())
    }

    fn migrate_v1_to_v2(conn: &mut Connection, directory: &Path) -> Result<(), MonoError> {
        let mut stmt = conn.prepare("SELECT id, title, type, date, content FROM notes")?;
        let notes = stmt
            .query_map(params![], parse_old_note)?
            .collect::<Result<Vec<OldNote>, _>>()?;

        drop(stmt);

        std::fs::create_dir_all(directory)?;
        let path = directory.canonicalize()?;

        let mut migrated_files = Vec::new();

        for note in notes {
            let slug = slugify(&note.title);
            let id = note.id;
            let filename = format!("{id:04}_{slug}.md");
            let full_path = path.join(filename);
            match std::fs::read_to_string(&full_path) {
                Ok(content) if content == note.content => {}
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    write_all(&full_path, note.content)?;
                }
                Ok(_) => {
                    return Err(MonoError::InvalidInput(format!(
                        "Migration would overwrite an existing note: {}",
                        full_path.display(),
                    )));
                }
                Err(err) => return Err(err.into()),
            }

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

        tx.pragma_update(None, "user_version", V2_SCHEMA_VERSION)?;

        tx.commit()?;

        Ok(())
    }

    fn migrate_v2_to_v3(conn: &mut Connection) -> Result<(), MonoError> {
        let tx = conn.transaction()?;

        tx.execute_batch("ALTER TABLE notes ADD COLUMN value INTEGER")?;
        tx.pragma_update(None, "user_version", CURRENT_SCHEMA_VERSION)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn legacy_db(version: u32) -> Result<Connection, MonoError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE TABLE notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                type TEXT DEFAULT 'other',
                date TEXT DEFAULT CURRENT_TIMESTAMP,
                content TEXT
            );
            INSERT INTO notes VALUES
                (7, 'Legacy note', 'check in', '2026-09-01 09:30:00.123456', 'Original body');",
        )?;
        conn.pragma_update(None, "user_version", version)?;
        Ok(conn)
    }

    fn version(conn: &Connection) -> Result<u32, rusqlite::Error> {
        conn.query_row("PRAGMA user_version", [], |row| row.get(0))
    }

    fn v2_db(stamped_version: u32) -> Result<Connection, MonoError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE TABLE notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                type TEXT DEFAULT 'other' CHECK (TYPE IN ('todo', 'check in', 'idea', 'other')),
                date TEXT DEFAULT CURRENT_TIMESTAMP,
                file_link TEXT
            );
            INSERT INTO notes (id, title, type, date, file_link) VALUES
                (2, 'V2 note', 'todo', '2026-10-01 08:00:00.000000', 'notes/0002_v2-note.md');",
        )?;
        conn.pragma_update(None, "user_version", stamped_version)?;
        Ok(conn)
    }

    #[test]
    fn migrates_v2_databases_to_v3() -> Result<(), MonoError> {
        for stamped_version in [1, 2, 3] {
            let mut conn = v2_db(stamped_version)?;
            Database::check_version(&mut conn)?;
            assert_eq!(version(&conn)?, 3);
            let (title, file_link, value): (String, String, Option<i64>) =
                conn.query_row("SELECT title, file_link, value FROM notes", [], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })?;
            assert_eq!(title, "V2 note");
            assert_eq!(file_link, "notes/0002_v2-note.md");
            assert_eq!(value, None);

            Database::check_version(&mut conn)?;
            assert_eq!(version(&conn)?, 3);
        }
        Ok(())
    }

    #[test]
    fn migrates_legacy_and_incorrectly_stamped_databases() -> Result<(), MonoError> {
        for old_version in [0, 1, 2] {
            let directory = tempfile::tempdir()?;
            let mut conn = legacy_db(old_version)?;
            Database::check_version_at(&mut conn, Some(directory.path()))?;
            assert_eq!(version(&conn)?, 3);
            assert!(conn.prepare("SELECT content FROM notes").is_err());
            let note = conn.query_row(
                "SELECT id, title, type, date, file_link, value FROM notes",
                [],
                crate::utils::parse_note,
            )?;
            assert_eq!(note.id, 7);
            assert_eq!(note.title, "Legacy note");
            assert_eq!(note.note_type, crate::types::NoteTypes::CheckIn);
            assert_eq!(note.date.to_string(), "2026-09-01 09:30:00.123456");
            assert!(note.file_link.is_absolute());
            assert_eq!(fs::read_to_string(&note.file_link)?, "Original body");
            let value: Option<i64> =
                conn.query_row("SELECT value FROM notes", [], |row| row.get(0))?;
            assert_eq!(value, None);

            fs::write(&note.file_link, "Edited outside mono")?;
            Database::check_version_at(&mut conn, Some(directory.path()))?;
            assert_eq!(fs::read_to_string(&note.file_link)?, "Edited outside mono");
        }
        Ok(())
    }

    #[test]
    fn initializes_fresh_and_repairs_converted_schema_versions() -> Result<(), MonoError> {
        let mut conn = Connection::open_in_memory()?;
        Database::check_version(&mut conn)?;
        assert_eq!(version(&conn)?, 3);
        for old_version in [0, 1, 2] {
            conn.pragma_update(None, "user_version", old_version)?;
            Database::check_version(&mut conn)?;
            assert_eq!(version(&conn)?, 3);
            assert!(conn.prepare("SELECT file_link FROM notes").is_ok());
            assert!(conn.prepare("SELECT value FROM notes").is_ok());
        }
        Ok(())
    }

    #[test]
    fn failed_migration_keeps_schema_and_content_and_can_retry() -> Result<(), MonoError> {
        let directory = tempfile::tempdir()?;
        let blocked = directory.path().join("blocked");
        fs::write(&blocked, "Not a directory")?;
        let mut conn = legacy_db(0)?;
        assert!(Database::check_version_at(&mut conn, Some(&blocked)).is_err());

        conn.execute_batch(
            "CREATE TRIGGER refuse_update BEFORE UPDATE ON notes
             BEGIN SELECT RAISE(ABORT, 'simulated failure'); END;",
        )?;
        assert!(Database::check_version_at(&mut conn, Some(directory.path())).is_err());
        assert_eq!(version(&conn)?, 0);
        let content: String = conn.query_row("SELECT content FROM notes", [], |row| row.get(0))?;
        assert_eq!(content, "Original body");
        assert!(conn.prepare("SELECT file_link FROM notes").is_err());

        conn.execute_batch("DROP TRIGGER refuse_update")?;
        Database::check_version_at(&mut conn, Some(directory.path()))?;
        assert_eq!(version(&conn)?, 3);
        Ok(())
    }

    #[test]
    fn migration_does_not_overwrite_conflicting_markdown() -> Result<(), MonoError> {
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("0007_legacy-note.md");
        fs::write(&path, "Keep these external edits")?;
        let mut conn = legacy_db(0)?;
        assert!(Database::check_version_at(&mut conn, Some(directory.path())).is_err());
        assert_eq!(version(&conn)?, 0);
        assert_eq!(fs::read_to_string(path)?, "Keep these external edits");
        Ok(())
    }

    #[test]
    fn rejects_future_schema_versions() -> Result<(), MonoError> {
        let mut conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        conn.pragma_update(None, "user_version", 4)?;
        assert!(Database::check_version(&mut conn).is_err());
        assert_eq!(version(&conn)?, 4);
        Ok(())
    }
}
