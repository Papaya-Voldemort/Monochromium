use crate::database::Database;

pub fn export(db: &Database) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rows = db.read_rows(-1, None)?;

    let mut output: Vec<String> = Vec::new();

    for note in rows {
        let push = format!(
            "{} \u{2022} ID: {} \u{2022} {}\n {}",
            note.title, note.id, note.date.format("%b %d, %Y at%l:%M %p"), note.content
        );
        output.push(push);
    }

    Ok(output)
}
