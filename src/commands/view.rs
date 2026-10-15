use crate::database::Database;
use crate::storage::{check_file_type, read_file};
use crate::types::MonoError;

pub fn view(db: &Database, node_id: u32, no_format: bool) -> Result<String, MonoError> {
    let note = db.read_single_row(node_id)?;

    if check_file_type(&note.file_link)? == "enc" {
        return Ok(format!(
            "This file is currently locked! Unlock it with `mono unlock {}`!",
            note.id
        ));
    }

    if no_format {
        return read_file(&note.file_link);
    }

    let content = read_file(&note.file_link)?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title,
        note.id,
        note.date.format("%b %d, %Y at%l:%M %p"),
        content
    );

    Ok(output)
}
