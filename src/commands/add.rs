use crate::config::load_config;
use crate::database::Database;
use crate::storage::write_all;
use crate::types::EditMode::FilePath;
use crate::types::{MonoError, NoteTypes};
use crate::utils::{make_title, slugify, string_check};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use std::path::PathBuf;

pub fn add(
    db: &Database,
    text: Option<String>,
    note_type: Option<NoteTypes>,
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    paste: bool,
) -> Result<String, MonoError> {
    let full_text = string_check(text, paste)?;

    if full_text.trim().is_empty() {
        return Err(MonoError::InvalidInput(
            "Note text cannot be empty".to_string(),
        ));
    }

    let now = Local::now();

    // Default values
    let time = time.unwrap_or(now.time());
    let date = date.unwrap_or(now.date_naive());
    let note_type = note_type.unwrap_or(NoteTypes::Other);

    let title = make_title(full_text.clone(), note_type.clone(), Some(time), Some(date));
    let datetime: NaiveDateTime = date.and_time(time);

    let temp_path = PathBuf::new();

    let id = db.add_row(title.clone(), note_type, temp_path, datetime)?;
    let config = load_config()?;

    let slug = slugify(&title);

    let filename = format!("{id:04}_{slug}.md");

    let full_path = config.note_location.join(filename);

    write_all(&full_path, full_text.clone())?;
    db.update_row(id, full_path.to_string_lossy().to_string(), FilePath)?;

    let note = db.read_single_row(id)?;

    let output = format!(
        "{}\n#{} · {} · {}\n{}\n\n{}",
        note.title,
        note.id,
        note.date.format("%b %-d, %Y"),
        note.date.format("%-I:%M %p"),
        note.file_link.display(),
        full_text,
    );

    Ok(output)
}
