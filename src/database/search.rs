use crate::commands::definitions::NoteTypes;
use crate::database::read::Note;
use crate::utils::parse_note_type;
use chrono::NaiveDateTime;
use libsql::params;

pub async fn search_notes(
    conn: libsql::Connection,
    search: Option<String>,
    limit: u16,
    note_type: Option<NoteTypes>,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let mut rows: libsql::Rows = match (search, note_type) {
        (Some(search), _) if !search.is_empty() => {
            let pattern = format!("%{}%", search);

            conn.query(
                "SELECT id, title, type, date, content
                 FROM notes
                 WHERE title LIKE ?1 OR content LIKE ?1
                 LIMIT ?2",
                params![pattern, limit],
            )
            .await?
        }
        (_, Some(note_type)) => {
            conn.query(
                "SELECT id, title, type, date, content
                 FROM notes
                 WHERE type = ?1
                 ORDER BY date DESC
                 LIMIT ?2",
                params![parse_note_type(note_type), limit],
            )
            .await?
        }
        _ => {
            conn.query(
                "SELECT id, title, type, date, content
                 FROM notes
                 ORDER BY date DESC
                 LIMIT ?2",
                params![limit],
            )
            .await?
        }
    };

    let mut parsed_rows = Vec::new();

    while let Some(row) = rows.next().await? {
        let date_str: String = row.get(3)?;
        let parsed_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f")?;
        let note = Note {
            id: row.get(0)?,
            title: row.get(1)?,
            note_type: row.get(2)?,
            date: parsed_date.to_string(),
            content: row.get(4)?,
        };

        parsed_rows.push(note);
    }

    Ok(parsed_rows)
}
