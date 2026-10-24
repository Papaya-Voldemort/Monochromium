#[cfg(test)]
use crate::database::Database;
use crate::types::{EditMode, MonoError};
use crate::types::{Note, NoteTypes};
use std::path::PathBuf;

#[test]
fn note_lifecycle_round_trip() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;

    let dt = chrono::Utc::now().naive_utc();

    let test_note_id = db.add_row(
        "test".to_string(),
        NoteTypes::Other,
        PathBuf::from("path/test.md"),
        dt,
        None,
    )?;

    let results = db.search_rows(Some("test".to_string()), 1, Some(NoteTypes::Other), None)?;

    let search = results.first().unwrap();

    let comp = Note {
        id: test_note_id,
        title: "test".to_string(),
        note_type: NoteTypes::Other,
        date: dt,
        file_link: PathBuf::from("path/test.md"),
        content: None,
        value: None,
    };

    assert_eq!(search, &comp);

    let _edit = db.update_row(test_note_id, "-test-".to_string(), EditMode::Title)?;

    let read = db.read_single_row(test_note_id)?;

    let after_edit = "-test-".to_string();

    assert_eq!(after_edit, read.title);

    let _delete = db.delete_note(test_note_id)?;

    let check_deleted = db.read_single_row(test_note_id);

    assert!(check_deleted.is_err());

    Ok(())
}

#[test]
fn value_round_trips_through_reads() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;

    let dt = chrono::Utc::now().naive_utc();

    let scored_id = db.add_row(
        "scored".to_string(),
        NoteTypes::CheckIn,
        PathBuf::from("path/scored.md"),
        dt,
        Some(3),
    )?;
    let unscored_id = db.add_row(
        "unscored".to_string(),
        NoteTypes::CheckIn,
        PathBuf::from("path/unscored.md"),
        dt,
        None,
    )?;

    assert_eq!(db.read_single_row(scored_id)?.value, Some(3));
    assert_eq!(db.read_single_row(unscored_id)?.value, None);

    let rows = db.read_rows(-1, Some(NoteTypes::CheckIn))?;
    assert!(rows.iter().any(|note| note.value == Some(3)));
    assert!(rows.iter().any(|note| note.value.is_none()));

    Ok(())
}
