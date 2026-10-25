use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::MonoError;
use crate::utils::{DATETIME_FORMAT, is_locked, locked_notice};

pub fn export(db: &Database) -> Result<Vec<String>, MonoError> {
    let rows = db.read_rows(-1, None, None, None)?;

    let mut output: Vec<String> = Vec::new();

    for note in rows {
        let content = if is_locked(&note.file_link)? {
            locked_notice(note.id)?
        } else {
            let Some(content) = read_file_if_exists(&note.file_link)? else {
                continue;
            };

            content
        };

        let value = note
            .value
            .map(|num| num.to_string())
            .unwrap_or_else(|| "None".to_string());

        let push = format!(
            "---\nid: {}\ndate: {}\nvalue: {}\nlink: {}\n---\n\n## {}\n\n{}\n\n---\n",
            note.id,
            note.date.format(DATETIME_FORMAT),
            value,
            note.file_link.display(),
            note.title,
            content
        );
        output.push(push);
    }

    Ok(output)
}
