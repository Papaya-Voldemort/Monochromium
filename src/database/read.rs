use crate::commands::definitions::NoteTypes;
use crate::utils::parse_note_type;
use chrono::NaiveDateTime;
use libsql::params;

pub struct Note {
    pub id: i64,
    pub title: String,
    pub note_type: String,
    pub date: String,
    pub content: String,
}
pub async fn read_rows(
    db: libsql::Database,
    limit: u16,
    note_type: Option<NoteTypes>,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let conn = db.connect()?;

    let mut rows = match note_type {
        Some(t) => {
            let type_str = parse_note_type(t);
            conn.query(
                "SELECT id, title, type, date, content FROM notes WHERE type = ?1 LIMIT ?2",
                params![type_str, limit],
            )
            .await?
        }
        None => {
            conn.query(
                "SELECT id, title, type, date, content FROM notes LIMIT ?1",
                params![limit],
            )
            .await?
        }
    };

    let mut parsed_rows = Vec::new();

    while let Some(row) = rows.next().await? {
        let date_str: String = row.get(3)?;
        let parsed_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f")?;
        let final_date = parsed_date.format("%b %d, %Y at%l:%M %p").to_string();
        let note = Note {
            id: row.get(0)?,
            title: row.get(1)?,
            note_type: row.get(2)?,
            date: final_date,
            content: row.get(4)?,
        };

        parsed_rows.push(note);
    }

    Ok(parsed_rows)
}
