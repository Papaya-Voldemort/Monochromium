use chrono::NaiveDateTime;
use libsql::params;

pub struct Note {
    pub id: i64,
    pub title: String,
    pub note_type: String,
    pub date: NaiveDateTime,
    pub content: String,
}
pub async fn read_rows(
    db: libsql::Database,
    limit: u16,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let conn = db.connect()?;

    let mut rows = conn
        .query("SELECT * from notes LIMIT ?1", params![limit])
        .await?;

    let mut parsed_rows = Vec::new();

    while let Some(row) = rows.next().await? {
        let date_str: String = row.get(3)?;
        let parsed_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S%.f")?;
        let note = Note {
            id: row.get(0)?,
            title: row.get(1)?,
            note_type: row.get(2)?,
            date: parsed_date,
            content: row.get(4)?,
        };

        parsed_rows.push(note);
    }

    Ok(parsed_rows)
}
