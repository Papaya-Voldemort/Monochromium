use crate::commands::definitions::NoteTypes;
use crate::database::read_rows;
use chrono::NaiveDate;

pub async fn list(
    conn: libsql::Connection,
    limit: Option<u16>,
    note_type: Option<NoteTypes>,
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
    let notes = read_rows(conn, final_limit as i32, note_type).await?;
    let mut output = Vec::new();

    if view {
        for note in notes {
            let push = format!(
                "{} \u{2022} ID: {} \u{2022} {}\n {}",
                note.title, note.id, note.date, note.content
            );
            output.push(push);
        }
    } else {
        for note in notes {
            let push = format!("{} \u{2022} ID: {}", note.title, note.id);
            output.push(push);
        }
    }

    Ok(output)
}
