use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::{MonoError, NoteTypes};
use crate::utils::is_locked;
use chrono::NaiveDate;

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

    let final_limit = limit.unwrap_or(10);
    let query = text.trim().to_lowercase();
    let notes = db.read_rows(-1, note_type)?;
    let mut output = Vec::new();

    for note in notes {
        if output.len() == final_limit as usize {
            break;
        }
        if date.is_some_and(|date| note.date.date() != date) {
            continue;
        }
        let title_matches = note.title.to_lowercase().contains(&query);

        let content_matches = if title_matches || is_locked(&note.file_link)? {
            false
        } else {
            read_file_if_exists(&note.file_link)?
                .is_some_and(|content| content.to_lowercase().contains(&query))
        };

        if !title_matches && !content_matches {
            continue;
        }
        let push = format!("{} \u{2022} ID: {}", note.title, note.id);
        output.push(push);
    }

    Ok(output)
}
