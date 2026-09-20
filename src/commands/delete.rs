use crate::database::Database;

pub fn delete(
    db: &Database,
    note_id: u32,
    approve: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if approve {
        let deleted = db.delete_note(note_id)?;
        if deleted == 0 {
            return Ok(format!("No note found with ID {}", note_id));
        }
        return Ok(format!("Deleted note with ID of {}", note_id));
    }
    Ok(format!(
        "Please pass the '--approve' to confirm the deletion of note {}",
        note_id
    ))
}
