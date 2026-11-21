use crate::{
    database::Database,
    types::{
        FieldUpdate,
        MonoError::{self, InvalidInput},
        NoteTypes::Todo,
    },
};

pub fn handle_todos(
    db: &Database,
    note_ids: Vec<u32>,
    check_off: bool,
) -> Result<Vec<String>, MonoError> {
    if note_ids.is_empty() {
        return Err(InvalidInput("No note ids provided as input".to_string()));
    }

    let notes = db.read_specific_rows(&note_ids)?;
    let mut output = Vec::new();

    for id in note_ids {
        if !notes.iter().any(|note| note.id == id) {
            output.push(format!("Note {} does not exist.", id));
        }
    }

    for note in notes {
        if note.note_type != Todo {
            output.push(format!(
                "Note {} is a {}, not a todo. Only todo's can be checked on/off.",
                note.id,
                note.note_type.as_str()
            ));
            continue;
        }

        match check_off {
            true => match note.value {
                Some(score) if score.is_zero() => output.push(format!(
                    "Note {} has a value of 0, so it can't be checked off.",
                    note.id
                )),
                Some(score) if score.is_positive() => {
                    db.update_row(note.id, FieldUpdate::Value(score.flip()))?;
                    output.push(format!("Checked off note {}.", note.id));
                }
                Some(_) => output.push(format!("Note {} is already checked off.", note.id)),
                None => output.push(format!("Note {} has no value to check off.", note.id)),
            },
            false => match note.value {
                Some(score) if score.is_zero() => output.push(format!(
                    "Note {} has a value of 0, so it can't be unchecked.",
                    note.id
                )),
                Some(score) if score.is_negative() => {
                    db.update_row(note.id, FieldUpdate::Value(score.flip()))?;
                    output.push(format!("Unchecked note {}.", note.id));
                }
                Some(_) => output.push(format!("Note {} is already unchecked.", note.id)),
                None => output.push(format!("Note {} has no value to check off.", note.id)),
            },
        }
    }

    Ok(output)
}
