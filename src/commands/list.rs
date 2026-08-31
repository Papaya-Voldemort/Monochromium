use crate::commands::definitions::NoteTypes;
use chrono::NaiveDate;

pub fn list(
    db: libsql::Database,
    limit: Option<u16>,
    node_type: Option<NoteTypes>,
    today: bool,
    since: Option<NaiveDate>,
    view: bool,
) {
    let conn = db.connect();
}
