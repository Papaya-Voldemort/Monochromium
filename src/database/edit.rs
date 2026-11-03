use super::Database;
use crate::types::EditMode;
use chrono::NaiveDateTime;
use rusqlite::params;

impl Database {
    pub fn update_row(
        &self,
        note_id: u32,
        change: String,
        mode: EditMode,
    ) -> Result<String, rusqlite::Error> {
        if mode == EditMode::Date {
            NaiveDateTime::parse_from_str(&change, "%Y-%m-%d %H:%M:%S%.f").map_err(|err| {
                rusqlite::Error::FromSqlConversionFailure(
                    3,
                    rusqlite::types::Type::Text,
                    Box::new(err),
                )
            })?;
        }

        let column: &str = match mode {
            EditMode::Title => "title",
            EditMode::FilePath => "file_link",
            EditMode::Date => "date",
            EditMode::Type => "type",
            EditMode::Value => "value",
        };

        let before: String = match mode {
            EditMode::Value => {
                let old: Option<i64> = self.conn.query_row(
                    "SELECT value FROM notes WHERE id = ?1",
                    params![note_id],
                    |row| row.get(0),
                )?;
                old.map(|v| v.to_string())
                    .unwrap_or_else(|| "none".to_string())
            }
            _ => self.conn.query_row(
                &format!("SELECT {} FROM notes WHERE id = ?1", column),
                params![note_id],
                |row| row.get(0),
            )?,
        };

        let query = format!("UPDATE notes SET {} = ?1 WHERE id = ?2", column);

        self.conn
            .execute(query.as_str(), params![change, note_id])?;

        Ok(before)
    }
}
