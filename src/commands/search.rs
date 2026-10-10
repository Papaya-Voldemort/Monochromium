use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use chrono::NaiveDate;

// TODO: Update to fuzzy search file contents
pub fn search(
    db: &Database,
    text: String,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    date: Option<NaiveDate>,
) -> Result<Vec<String>, MonoError> {
    if text.trim().is_empty() {
        return Err(MonoError::InvalidInput(
            "Please input a query for your search".to_string(),
        ));
    }

    let datetime = date.and_then(|d| d.and_hms_opt(0, 0, 0));

    let final_limit = limit.unwrap_or(10);
    let notes = db.search_rows(
        Some(text.trim().to_string()),
        final_limit,
        note_type,
        datetime,
    )?;

    let mut output = Vec::new();

    for note in notes {
        let push = format!("{} \u{2022} ID: {}", note.title, note.id);
        output.push(push);
    }

    Ok(output)
}
