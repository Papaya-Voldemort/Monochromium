use crate::database::{Database, UpdateType};

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
) -> Result<Output, Box<dyn std::error::Error>> {
    let new = text.clone();

    let update_type = if append {
        UpdateType::Append
    } else if overwrite {
        UpdateType::Overwrite
    } else {
        return Err("Must specify either append or overwrite".into());
    };

    let old = db.update_row(note_id, text, update_type)?;

    Ok(Output { old, new })
}
