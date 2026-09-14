use crate::database::read_single_row;

pub async fn view(
    conn: libsql::Connection,
    node_id: u32,
    _no_format: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let note = read_single_row(conn, node_id).await?;

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title, note.id, note.date, note.content
    );

    Ok(output)
}
