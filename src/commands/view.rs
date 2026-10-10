use crate::database::Database;
use crate::storage::read_file;
use crate::types::MonoError;

pub fn view(db: &Database, node_id: u32, no_format: bool) -> Result<String, MonoError> {
    let mut note = db.read_single_row(node_id)?;

    if no_format {
        return read_file(&note.file_link);
    }

    note.content = Some(read_file(&note.file_link)?);

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title,
        note.id,
        note.date.format("%b %d, %Y at%l:%M %p"),
        note.content.unwrap()
    );

    Ok(output)
}
