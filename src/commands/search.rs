use crate::commands::definitions::NoteTypes;
use crate::database::search_notes;
use chrono::NaiveDate;
use libsql::Op;
use tokio::io;

pub async fn search(
    conn: libsql::Connection,
    text: String,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
    date: Option<NaiveDate>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let final_limit: u16;
    if limit == None {
        final_limit = 10;
    } else {
        final_limit = limit.unwrap();
    }
    let notes = search_notes(conn, text, final_limit).await?;

    let mut output = Vec::new();

    for note in notes {
        let push = format!("{} \u{2022} ID: {}", note.title, note.id);
        output.push(push);
    }

    Ok(output)
}
