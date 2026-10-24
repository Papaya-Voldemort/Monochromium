use super::{delete, edit, export, list, search};
use crate::database::Database;
use crate::types::{EditMode, MonoError, NoteTypes};
use chrono::{Duration, Local, NaiveDate};
use std::{fs, path::Path};

fn note(
    db: &Database,
    directory: &Path,
    title: &str,
    content: &str,
    kind: NoteTypes,
    date: NaiveDate,
) -> Result<u32, MonoError> {
    let path = directory.join(format!("{title}.md"));
    fs::write(&path, content)?;
    Ok(db.add_row(
        title.to_string(),
        kind,
        path,
        date.and_hms_opt(9, 30, 0).unwrap(),
        None,
    )?)
}

#[test]
fn search_matches_bodies_and_filters_before_limiting() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let yesterday = today - Duration::days(1);
    let id = note(
        &db,
        directory.path(),
        "older",
        "BodyOnlyWord",
        NoteTypes::Idea,
        yesterday,
    )?;
    note(
        &db,
        directory.path(),
        "newer",
        "Unrelated",
        NoteTypes::Idea,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "other",
        "BodyOnlyWord",
        NoteTypes::Other,
        today,
    )?;
    let results = search(
        &db,
        "bodyonlyword".to_string(),
        Some(1),
        Some(NoteTypes::Idea),
        None,
    )?;
    assert_eq!(results, [format!("older • ID: {id}")]);
    assert_eq!(
        search(&db, "BODYONLYWORD".to_string(), None, None, Some(yesterday))?,
        results
    );
    assert!(search(&db, "bodyonlyword".to_string(), Some(0), None, None)?.is_empty());
    Ok(())
}

#[test]
fn list_applies_today_and_since_filters() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    note(
        &db,
        directory.path(),
        "yesterday",
        "Old",
        NoteTypes::Other,
        today - Duration::days(1),
    )?;
    let id = note(
        &db,
        directory.path(),
        "today",
        "Current",
        NoteTypes::Other,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "tomorrow",
        "Future",
        NoteTypes::Other,
        today + Duration::days(1),
    )?;
    assert_eq!(
        list(&db, Some(1), None, true, None, false)?,
        [format!("today • ID: {id}")]
    );
    let results = list(&db, None, None, false, Some(today), false)?;
    assert_eq!(results.len(), 2);
    assert!(!results.iter().any(|line| line.contains("yesterday")));
    Ok(())
}

#[test]
fn deletion_failures_preserve_database_row_and_file() -> Result<(), MonoError> {
    for fail_commit in [false, true] {
        let db = Database::new_in_memory()?;
        let directory = tempfile::tempdir()?;
        let id = note(
            &db,
            directory.path(),
            "test",
            "Original body",
            NoteTypes::Other,
            Local::now().date_naive(),
        )?;
        if fail_commit {
            db.conn.execute_batch(
                "PRAGMA foreign_keys = ON;
                 CREATE TABLE dependencies (note_id INTEGER REFERENCES notes(id) DEFERRABLE INITIALLY DEFERRED);
                 INSERT INTO dependencies VALUES (1);",
            )?;
        } else {
            db.conn.execute_batch(
                "CREATE TRIGGER refuse_delete BEFORE DELETE ON notes
                 BEGIN SELECT RAISE(ABORT, 'simulated failure'); END;",
            )?;
        }
        assert!(delete(&db, id, true).is_err());
        let row = db.read_single_row(id)?;
        assert_eq!(fs::read_to_string(row.file_link)?, "Original body");
        assert_eq!(fs::read_dir(directory.path())?.count(), 1);
    }
    Ok(())
}

#[test]
fn deletion_removes_notes_and_handles_missing_files() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    for missing in [false, true] {
        let id = note(
            &db,
            directory.path(),
            "test",
            "Body",
            NoteTypes::Other,
            Local::now().date_naive(),
        )?;
        if missing {
            fs::remove_file(db.read_single_row(id)?.file_link)?;
        }
        delete(&db, id, true)?;
        assert!(db.read_single_row(id).is_err());
        assert_eq!(fs::read_dir(directory.path())?.count(), 0);
    }
    Ok(())
}

#[test]
fn editing_normalizes_types_and_rejects_missing_file_links() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let id = note(
        &db,
        directory.path(),
        "test",
        "Body",
        NoteTypes::Other,
        Local::now().date_naive(),
    )?;
    for alias in ["check-in", "checkin", "check in"] {
        edit(&db, id, Some(EditMode::Type), Some(alias.to_string()))?;
        assert_eq!(db.read_single_row(id)?.note_type, NoteTypes::CheckIn);
    }
    let before = db.read_single_row(id)?.file_link;
    assert!(
        edit(
            &db,
            id,
            Some(EditMode::FilePath),
            Some(
                directory
                    .path()
                    .join("missing.md")
                    .to_string_lossy()
                    .to_string()
            )
        )
        .is_err()
    );
    assert_eq!(db.read_single_row(id)?.file_link, before);
    Ok(())
}

#[test]
fn missing_files_do_not_abort_bulk_commands() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;
    let directory = tempfile::tempdir()?;
    let today = Local::now().date_naive();
    let missing = note(
        &db,
        directory.path(),
        "missing",
        "Gone",
        NoteTypes::Other,
        today,
    )?;
    note(
        &db,
        directory.path(),
        "healthy",
        "Kept content",
        NoteTypes::Other,
        today,
    )?;
    fs::remove_file(db.read_single_row(missing)?.file_link)?;
    let output = export(&db)?;
    assert_eq!(output.len(), 1);
    assert!(output[0].contains("Kept content"));
    assert_eq!(list(&db, None, None, false, None, true)?.len(), 2);
    assert_eq!(
        search(&db, "Kept content".to_string(), None, None, None)?.len(),
        1
    );
    Ok(())
}

#[test]
fn value_flag_accepts_negative_and_rejects_out_of_range() {
    use crate::types::{MonoCLI, MonoCommands};
    use clap::Parser;

    let cli = MonoCLI::try_parse_from(["mono", "add", "text", "-v", "-3"]).unwrap();
    match cli.command {
        Some(MonoCommands::Add { value, .. }) => assert_eq!(value, Some(-3)),
        _ => panic!("expected add command"),
    }

    assert!(MonoCLI::try_parse_from(["mono", "add", "text", "-v", "6"]).is_err());
    assert!(MonoCLI::try_parse_from(["mono", "add", "text", "-v", "-6"]).is_err());
}
