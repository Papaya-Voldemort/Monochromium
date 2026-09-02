use crate::database::delete_note;
use tokio::io;

pub async fn delete(
    conn: libsql::Connection,
    note_id: u32,
    approve: bool,
) -> Result<String, libsql::Error> {
    if approve {
        let deleted = delete_note(conn, note_id).await?;
        if deleted == 0 {
            return Ok(format!("No note found with ID {}", note_id))
        }
        return Ok(format!("Deleted note with ID of {}", note_id));
    }
    Ok(format!(
        "Please pass the '--approve' to confirm the deletion of note {}",
        note_id
    ))
}
