use crate::database::Database;
use crate::storage::read_file_if_exists;
use crate::types::MonoError;

pub fn export(db: &Database) -> Result<Vec<String>, MonoError> {
    let rows = db.read_rows(-1, None)?;

    let mut output: Vec<String> = Vec::new();

    for note in rows {
        let Some(content) = read_file_if_exists(&note.file_link)? else {
            continue;
        };
        let push = format!(
            "## {}\n\n**#{}** · {}  \n`{}`\n\n{}\n\n---\n",
            note.title,
            note.id,
            note.date.format("%b %-d, %Y at %-I:%M %p"),
            note.file_link.display(),
            content,
        );
        output.push(push);
    }

    Ok(output)
}
