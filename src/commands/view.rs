use crate::database::Database;
use crate::types::MonoError;

pub fn view(db: &Database, node_id: u32, _no_format: bool) -> Result<String, MonoError> {
    let note = db.read_single_row(node_id)?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title,
        note.id,
        note.date.format("%b %d, %Y at%l:%M %p"),
        note.content
    );

    Ok(output)
}
