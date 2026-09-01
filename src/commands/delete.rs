use crate::database::delete_note;

pub async fn delete(conn: libsql::Connection, note_id: u32, approve: bool) -> String {
    if approve {
        let delete = delete_note(conn, note_id).await;
        return format!("Deleted note with ID of {}", note_id);
    }
    return format!(
        "Please pass the '--approve' to confirm the deletion of note {}",
        note_id
    );
}
