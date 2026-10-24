use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::MonoError;
use crate::utils::{DATETIME_FORMAT, is_locked, locked_notice};

pub fn export(db: &Database) -> Result<Vec<String>, MonoError> {
    let rows = db.read_rows(-1, None)?;

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

        let push = format!(
            "## {}\n\n**#{}** · {}  \n`{}`\n\n{}\n\n---\n",
            note.title,
            note.id,
            note.date.format(DATETIME_FORMAT),
            note.file_link.display(),
            content,
        );
        output.push(push);
    }

    Ok(output)
}
