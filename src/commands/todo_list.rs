use crate::database::read_rows;
use crate::types::NoteTypes;
use crate::utils::pretty_notes;
use chrono::NaiveDate;

pub async fn list_todos(
    conn: libsql::Connection,
    limit: Option<u16>,
    _today: bool,
    _since: Option<NaiveDate>,
    view: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let limit = limit.unwrap_or(10);
    let notes = read_rows(conn, limit as i32, Some(NoteTypes::Todo)).await?;
    Ok(pretty_notes(notes, view))
}
