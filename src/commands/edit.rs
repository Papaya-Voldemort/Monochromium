use crate::database::Database;
use crate::storage::{edit_file, read_file};
use crate::types::{EditMode, FieldUpdate, MonoError, Note, NoteTypes, Score};
use crate::utils::{DATETIME_FORMAT, is_locked, locked_notice};
use chrono::NaiveDateTime;

const DATE_INPUT_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

pub struct Output {
    pub _old: Option<String>,
    pub _new: Option<String>,
    pub _update_type: Option<EditMode>,
    pub output: String,
}

pub fn edit(
    db: &Database,
    note_id: u32,
    update_type: Option<EditMode>,
    text: Option<String>,
) -> Result<Output, MonoError> {
    let Some(update_type) = update_type else {
        return edit_content(db, note_id);
    };

    let text = text.ok_or_else(|| MonoError::InvalidInput("Missing text content".to_string()))?;
    let update = parse_update(update_type, text)?;

    let result = db.update_row(note_id, update)?;

    let old = field_value(&result.before, &result.change);
    let new = field_value(&result.after, &result.change);

    let output = format!(
        "Replaced old {} \"{}\" with \"{}\"!",
        field_name(&result.change),
        old,
        new
    );

    Ok(Output {
        _old: Some(old),
        _new: Some(new),
        _update_type: Some(update_type),
        output,
    })
}

fn edit_content(db: &Database, note_id: u32) -> Result<Output, MonoError> {
    let old_note = db.read_single_row(note_id)?;
    if is_locked(&old_note.file_link)? {
        return Ok(Output {
            _old: None,
            _new: None,
            _update_type: None,
            output: locked_notice(old_note.id)?,
        });
    }

    let path = old_note.file_link;

    let old_content = read_file(&path)?;

    let saved = edit_file(&path)?;

    let new_content = read_file(&path)?;

    let out = if saved {
        "Edit made successfully!"
    } else {
        "No changes saved."
    }
    .to_string();

    Ok(Output {
        _old: Some(old_content),
        _new: Some(new_content),
        _update_type: None,
        output: out,
    })
}

fn parse_update(update_type: EditMode, text: String) -> Result<FieldUpdate, MonoError> {
    match update_type {
        EditMode::Title => {
            if text.trim().is_empty() {
                return Err(MonoError::InvalidInput("Title cannot be empty".to_string()));
            }
            Ok(FieldUpdate::Title(text))
        }
        EditMode::FilePath => {
            let path = std::path::Path::new(&text);
            if is_locked(path)? {
                return Err(MonoError::InvalidInput(
                    "You can't change the path to an encrypted file".to_string(),
                ));
            }
            if !path.is_file() {
                return Err(MonoError::InvalidInput(
                    "File path must point to an existing note file".to_string(),
                ));
            }
            read_file(&path.to_path_buf())?;
            Ok(FieldUpdate::FilePath(
                path.canonicalize()?.to_string_lossy().to_string(),
            ))
        }
        EditMode::Type => Ok(FieldUpdate::NoteType(
            text.parse::<NoteTypes>()?.as_str().to_string(),
        )),
        EditMode::Date => Ok(FieldUpdate::Date(
            NaiveDateTime::parse_from_str(&text, DATE_INPUT_FORMAT).map_err(|_| {
                MonoError::InvalidInput(
                    "Date must be in the format YYYY-MM-DD HH:MM:SS".to_string(),
                )
            })?,
        )),
        EditMode::Value => Ok(FieldUpdate::Value(Score::parse(&text)?)),
    }
}

fn field_name(update: &FieldUpdate) -> &'static str {
    match update {
        FieldUpdate::Title(_) => "title",
        FieldUpdate::FilePath(_) => "file path",
        FieldUpdate::NoteType(_) => "type",
        FieldUpdate::Date(_) => "date",
        FieldUpdate::Value(_) => "value",
    }
}

fn field_value(note: &Note, update: &FieldUpdate) -> String {
    match update {
        FieldUpdate::Title(_) => note.title.clone(),
        FieldUpdate::FilePath(_) => note.file_link.display().to_string(),
        FieldUpdate::NoteType(_) => note.note_type.to_string(),
        FieldUpdate::Date(_) => note.date.format(DATETIME_FORMAT).to_string(),
        FieldUpdate::Value(_) => note
            .value
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
    }
}
