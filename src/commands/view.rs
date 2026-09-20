use crate::database::Database;

pub fn view(
    db: &Database,
    node_id: u32,
    _no_format: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let note = db.read_single_row(node_id)?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title, note.id, note.date, note.content
    );

    Ok(output)
}
