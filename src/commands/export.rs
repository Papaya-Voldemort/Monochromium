use crate::database::read_rows;

pub async fn export(conn: libsql::Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rows = read_rows(conn, -1, None).await?;

    let mut output: Vec<String> = Vec::new();

    for note in rows {
        let push = format!(
            "{} \u{2022} ID: {} \u{2022} {}\n {}",
            note.title, note.id, note.date, note.content
        );
        output.push(push);
    }

    Ok(output)
}
