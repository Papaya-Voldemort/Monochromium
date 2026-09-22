use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use chrono::NaiveDate;

pub fn search(
    db: &Database,
    text: String,
    limit: Option<u16>,
    _note_type: Option<NoteTypes>,
    _date: Option<NaiveDate>,
) -> Result<Vec<String>, MonoError> {
    if text.trim().is_empty() {
        return Err(MonoError::InvalidInput(
            "Please input a query for your search".to_string(),
        ));
    }

    let final_limit = limit.unwrap_or(10);
    let notes = db.search_rows(Some(text), final_limit, None)?;

    let mut output = Vec::new();

    for note in notes {
        let push = format!("{} \u{2022} ID: {}", note.title, note.id);
        output.push(push);
    }

    Ok(output)
}
