use libsql::params;

pub async fn delete_note(conn: libsql::Connection, note_id: u32) -> Result<u64, libsql::Error> {
    let result = conn
        .execute("DELETE FROM notes WHERE id = ?1", params![note_id])
        .await;

    result
}
