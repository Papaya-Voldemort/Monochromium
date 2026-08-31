use crate::commands::definitions::NoteTypes;
use chrono::NaiveDate;
use crate::database::read_rows;

pub async fn list(
    db: libsql::Database,
    limit: Option<u16>,
    node_type: Option<NoteTypes>,
    today: bool,
    since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let final_limit: u16;
    if limit == None {
        final_limit = 15;
    } else {
        final_limit = limit.unwrap();
    }
    let notes = read_rows(db, final_limit).await?;
    let mut output = Vec::new();

    for note in notes {
        let push = format!("{} \u{2022} ID: {}", note.title, note.id);
        output.push(push);
    }

    Ok(output)
}
