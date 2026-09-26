use crate::database::Database;
use crate::types::MonoError;
use crate::types::NoteTypes::{CheckIn, Idea, Other, Todo};

pub fn get_stats(db: &Database) -> Result<String, MonoError> {
    let total_notes = db.read_rows(-1, None)?.len();
    let total_checkins = db.read_rows(-1, Some(CheckIn))?.len();
    let total_todos = db.read_rows(-1, Some(Todo))?.len();
    let total_ideas = db.read_rows(-1, Some(Idea))?.len();
    let total_other = db.read_rows(-1, Some(Other))?.len();

    Ok(format!(
        "\
Monochromium Stats

  {:<15} {}
  {:<15} {}
  {:<15} {}
  {:<15} {}
  {:<15} {}
",
        "Total notes",
        total_notes,
        "Check-ins",
        total_checkins,
        "Todos",
        total_todos,
        "Ideas",
        total_ideas,
        "Other",
        total_other,
    ))
}
