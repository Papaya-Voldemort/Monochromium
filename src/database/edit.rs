use libsql::params;

pub enum UpdateType {
    Append,
    Overwrite,
    None,
}

pub async fn update_note(
    conn: libsql::Connection,
    note_id: u32,
    text: String,
    mode: UpdateType,
) -> Result<String, libsql::Error> {
    let mut rows = conn
        .query("SELECT content FROM notes WHERE id = ?1", params!(note_id))
        .await?;

    let pre: String = if let Some(row) = rows.next().await? {
        row.get(0)?
    } else {
        return Err(libsql::Error::QueryReturnedNoRows);
    };

    let update = match mode {
        UpdateType::Append => {
            conn.execute(
                "UPDATE notes SET content = content || ?1 WHERE id = ?2;",
                params!(text, note_id),
            )
            .await?;
        }
        UpdateType::Overwrite => {
            conn.execute(
                "UPDATE notes SET content = ?1 WHERE id = ?2;",
                params!(text, note_id),
            )
            .await?;
        }
        UpdateType::None => {}
    };

    Ok(pre)
}
