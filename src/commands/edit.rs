use crate::database::{Database, UpdateType};
use crate::types::MonoError;

pub struct Output {
    pub old: String,
    pub new: String,
}

pub fn edit(
    db: &Database,
    note_id: u32,
    _headless: bool,
    append: bool,
    overwrite: bool,
    text: String,
) -> Result<Output, MonoError> {
    let new = text.clone();

    let update_type = if append {
        UpdateType::Append
    } else if overwrite {
        UpdateType::Overwrite
    } else {
        return Err(MonoError::InvalidInput(
            "Must specify either append or overwrite".into(),
        ));
    };

    let old = db.update_row(note_id, text, update_type)?;

    Ok(Output { old, new })
}
