use crate::{database::Database, types::MonoError};

pub fn done(db: &Database, note_ids: Vec<u32>) -> Result<String, MonoError> {
    db.read_specific_rows(&note_ids)?;

    Ok("todo".to_string())
}
