use crate::database::read_single_row;

pub async fn view(conn: libsql::Connection, node_id: u32, no_format: bool) -> String {
    let note = read_single_row(conn, node_id).await.unwrap();

    let output = format!(
        "{} \u{2022} ID: {} \u{2022} {}\n {}",
        note.title, note.id, note.date, note.content
    );

    output
}
