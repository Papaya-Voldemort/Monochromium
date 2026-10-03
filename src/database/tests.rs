#[cfg(test)]
use crate::database::Database;
use crate::types::{EditMode, MonoError};
use crate::types::{OldNote, NoteTypes};

#[test]
fn note_lifecycle_round_trip() -> Result<(), MonoError> {
    let db = Database::new_in_memory()?;

    let dt = chrono::Utc::now().naive_utc();

    let test_note_id = db.add_row(
        "test".to_string(),
        NoteTypes::Other,
        "text note content".to_string(), // TODO: Fix this to be a path eventually, path probably does not need to be real or make a real on device note md file but up to you
        dt,
    )?;

    let results = db.search_rows(Some("test".to_string()), 1, Some(NoteTypes::Other))?;

    let search = results.first().unwrap();

    let comp = OldNote {
        id: test_note_id,
        title: "test".to_string(),
        _note_type: "other".to_string(),
        date: dt,
        content: "text note content".to_string(),
    };

    assert_eq!(search, &comp);

    let edit = db.update_row(test_note_id, "APPEND".to_string(), EditMode::Append)?;

    let read = db.read_single_row(test_note_id)?;

    let after_edit = format!("{} APPEND", edit);

    assert_eq!(after_edit, read.content);

    let _delete = db.delete_note(test_note_id);

    let check_deleted = db.read_single_row(test_note_id);

    assert!(check_deleted.is_err());

    Ok(())
}
