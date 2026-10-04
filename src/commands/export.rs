use crate::database::Database;
use crate::storage::read_file;
use crate::types::MonoError;
use std::path::PathBuf;

pub fn export(db: &Database) -> Result<Vec<String>, MonoError> {
    let rows = db.read_rows(-1, None)?;

    let mut output: Vec<String> = Vec::new();

    for note in rows {
        let content = read_file(&PathBuf::from(&note.file_link))?;
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
