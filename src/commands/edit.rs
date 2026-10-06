use crate::database::Database;
use crate::storage::{edit_file, read_file};
use crate::types::{EditMode, MonoError};

pub struct Output {
    pub _old: Option<String>,
    pub _new: Option<String>,
    pub _update_type: Option<EditMode>,
    pub output: String,
}

// TODO: Fix this function in 0.5.x to be more functional and use more intuitive intended behaviors
pub fn edit(
    db: &Database,
    note_id: u32,
    update_type: Option<EditMode>,
    text: Option<String>,
) -> Result<Output, MonoError> {
    if update_type.is_some() {
        let update_type = update_type
            .ok_or_else(|| MonoError::InvalidInput("Missing update type".to_string()))?;

        let text =
            text.ok_or_else(|| MonoError::InvalidInput("Missing text content".to_string()))?;

        let old = db.update_row(note_id, text.clone(), update_type)?;

        let out = match update_type {
            EditMode::Title => {
                format!("Replaced old title \"{}\" with \"{}\"!", old, text)
            }
            EditMode::FilePath => {
                format!("Replaced old file path \"{}\" with \"{}\"!", old, text)
            }
            EditMode::Type => {
                format!("Replaced old type \"{}\" with \"{}\"!", old, text)
            }
            EditMode::Date => {
                format!("Replaced old date \"{}\" with \"{}\"!", old, text)
            }
        };

        Ok(Output {
            _old: Some(old),
            _new: Some(text),
            _update_type: Some(update_type),
            output: out,
        })
    } else {
        let old_note = db.read_single_row(note_id)?;
        let path = old_note.file_link;

        let old_content = read_file(&path)?;

        edit_file(&path)?;

        let new_content = read_file(&path)?;

        let out = "Edit made successfully!".to_string();

        Ok(Output {
            _old: Some(old_content),
            _new: Some(new_content),
            _update_type: None,
            output: out,
        })
    }
}
