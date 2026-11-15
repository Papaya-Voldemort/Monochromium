use super::Database;
use crate::types::{FieldUpdate, UpdateResult};
use crate::utils::parse_note;
use rusqlite::params;

impl Database {
    pub fn update_row(
        &self,
        note_id: u32,
        update: FieldUpdate,
    ) -> Result<UpdateResult, rusqlite::Error> {
        let column = update.key();

        let before = self.conn.query_row(
            "SELECT * FROM notes WHERE id = ?1",
            params![note_id],
            parse_note,
        )?;

        let sql = format!("UPDATE notes SET {column} = ?1 WHERE id = ?2");
        let changed = self
            .conn
            .execute(sql.as_str(), params![update.sql_value(), note_id])?;
        if changed != 1 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let after = self.conn.query_row(
            "SELECT * FROM notes WHERE id = ?1",
            params![note_id],
            parse_note,
        )?;

        Ok(UpdateResult {
            change: update,
            before,
            after,
        })
    }
}
