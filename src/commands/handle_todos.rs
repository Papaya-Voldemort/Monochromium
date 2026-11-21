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
    let notes = db.read_specific_rows(&note_ids)?;
    let mut output = Vec::new();

    if note_ids.is_empty() {
        return Err(InvalidInput("No note ids provided as input".to_string()));
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
                Some(score) if score.is_positive() => {
                    db.update_row(note.id, FieldUpdate::Value(score.flip()))?;
                    output.push(format!("Checked off note {}.", note.id));
                }
                Some(_) => output.push(format!("Note {} is already checked off.", note.id)),
                None => output.push(format!("Note {} has no value to check off.", note.id)),
            },
            false => match note.value {
                Some(score) if score.is_negetive() => {
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
