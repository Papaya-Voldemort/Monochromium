use super::Database;
use rusqlite::params;

pub enum UpdateType {
    Append,
    Overwrite,
    _None,
}

impl Database {
    pub fn update_row(
        &self,
        note_id: u32,
        text: String,
        mode: UpdateType,
    ) -> Result<String, rusqlite::Error> {
        let pre: String = self.conn.query_row(
            "SELECT content FROM notes WHERE id = ?1",
            params![note_id],
            |row| row.get(0),
        )?;

        match mode {
            UpdateType::Append => {
                self.conn.execute(
                    "UPDATE notes SET content = content || ?1 WHERE id = ?2;",
                    params![text, note_id],
                )?;
            }
            UpdateType::Overwrite => {
                self.conn.execute(
                    "UPDATE notes SET content = ?1 WHERE id = ?2;",
                    params![text, note_id],
                )?;
            }
            UpdateType::_None => {}
        };

        Ok(pre)
    }
}
