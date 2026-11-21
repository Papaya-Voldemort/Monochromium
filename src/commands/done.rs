use crate::{
    database::Database,
    types::{FieldUpdate, MonoError, NoteTypes::Todo},
};

pub fn done(db: &Database, note_ids: Vec<u32>) -> Result<Vec<String>, MonoError> {
    let notes = db.read_specific_rows(&note_ids)?;
    let mut output = Vec::new();

    for note in notes {
        if note.note_type != Todo {
            output.push(format!(
                "Note {} is a {}, not a todo. Only todo's can be checked off.",
                note.id,
                note.note_type.as_str()
            ));
            continue;
        }

        match note.value {
            Some(score) if score.is_positive() => {
                db.update_row(note.id, FieldUpdate::Value(score.flip()))?;
                output.push(format!("Checked off note {}.", note.id));
            }
            Some(_) => output.push(format!("Note {} is already checked off.", note.id)),
            None => output.push(format!("Note {} has no value to check off.", note.id)),
        }
    }

    Ok(output)
}
