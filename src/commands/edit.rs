use crate::database::Database;
use crate::types::{EditMode, MonoError};

pub struct Output {
    pub old: String,
    pub new: String,
    pub update_type: EditMode,
}

pub fn edit(
    db: &Database,
    note_id: u32,
    _headless: bool,
    update_type: Option<EditMode>,
    text: Option<String>,
) -> Result<Output, MonoError> {
    let update_type =
        update_type.ok_or_else(|| MonoError::InvalidInput("Missing update type".to_string()))?;

    let text = text.ok_or_else(|| MonoError::InvalidInput("Missing text content".to_string()))?;

    if update_type == EditMode::Append && text.trim().is_empty() {
        return Err(MonoError::InvalidInput(
            "Please insert a value to append".to_string(),
        ));
    }

    let old = db.update_row(note_id, text.clone(), update_type)?;

    Ok(Output {
        old,
        new: text,
        update_type,
    })
}
