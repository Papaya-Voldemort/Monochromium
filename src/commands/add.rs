use crate::config::load_config;
use crate::database::Database;
use crate::storage::{delete_file, write_all};
use crate::types::EditMode::FilePath;
use crate::types::{MonoError, Note, NoteTypes};
use crate::utils::{DATE_FORMAT, TIME_FORMAT, make_title, slugify, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use std::path::{Path, PathBuf};

pub fn add(
    db: &Database,
    text: Option<String>,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> Result<String, MonoError> {
    let full_text = string_check(text, paste)?;

    if full_text.trim().is_empty() {
        return Err(MonoError::InvalidInput(
            "Note text cannot be empty".to_string(),
        ));
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());
    let note_type = note_type.unwrap_or(NoteTypes::Other);

    let title = make_title(full_text.clone(), note_type.clone(), Some(time), Some(date));
    let datetime: NaiveDateTime = date.and_time(time);

    let config = load_config()?;
    let note = save_note(
        db,
        title,
        note_type,
        datetime,
        &full_text,
        &config.note_location,
    )?;

    let output = format!(
        "{}\n#{} · {} · {}\n{}\n\n{}",
        note.title,
        note.id,
        note.date.format(DATE_FORMAT),
        note.date.format(TIME_FORMAT),
        note.file_link.display(),
        full_text,
    );

    Ok(output)
}

fn save_note(
    db: &Database,
    title: String,
    note_type: NoteTypes,
    date: NaiveDateTime,
    content: &str,
    directory: &Path,
) -> Result<Note, MonoError> {
    std::fs::create_dir_all(directory)?;
    let directory = directory.canonicalize()?;
    let tx = db.conn.unchecked_transaction()?;
    let id = db.add_row(title.clone(), note_type, PathBuf::new(), date)?;
    let path = directory.join(format!("{id:04}_{}.md", slugify(&title)));
    write_all(&path, content)?;

    let result = (|| {
        db.update_row(id, path.to_string_lossy().to_string(), FilePath)?;
        let note = db.read_single_row(id)?;
        tx.commit()?;
        Ok::<_, MonoError>(note)
    })();

    if result.is_err() {
        delete_file(path)?;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_add_does_not_leave_a_row_or_overwrite_existing_content() -> Result<(), MonoError> {
        let db = Database::new_in_memory()?;
        let directory = tempfile::tempdir()?;
        let path = directory.path().join("0001_test.md");
        std::fs::write(&path, "Existing content")?;
        let result = save_note(
            &db,
            "test".to_string(),
            NoteTypes::Other,
            Local::now().naive_local(),
            "New content",
            directory.path(),
        );
        assert!(result.is_err());
        assert!(db.read_rows(-1, None)?.is_empty());
        assert_eq!(std::fs::read_to_string(path)?, "Existing content");
        Ok(())
    }

    #[test]
    fn failed_database_update_removes_the_new_file_and_row() -> Result<(), MonoError> {
        let db = Database::new_in_memory()?;
        db.conn.execute_batch(
            "CREATE TRIGGER refuse_update BEFORE UPDATE ON notes
             BEGIN SELECT RAISE(ABORT, 'simulated failure'); END;",
        )?;
        let directory = tempfile::tempdir()?;
        assert!(
            save_note(
                &db,
                "test".to_string(),
                NoteTypes::Other,
                Local::now().naive_local(),
                "New content",
                directory.path()
            )
            .is_err()
        );
        assert!(db.read_rows(-1, None)?.is_empty());
        assert_eq!(std::fs::read_dir(directory.path())?.count(), 0);
        Ok(())
    }
}
