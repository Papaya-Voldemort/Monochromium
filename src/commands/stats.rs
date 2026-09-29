use crate::database::Database;
use crate::types::MonoError;
use crate::types::NoteTypes::{CheckIn, Idea, Other, Todo};

pub fn get_stats(db: &Database) -> Result<String, MonoError> {
    let total_notes = db.read_rows(-1, None)?.len();
    let total_checkins = db.read_rows(-1, Some(CheckIn))?.len();
    let total_todos = db.read_rows(-1, Some(Todo))?.len();
    let total_ideas = db.read_rows(-1, Some(Idea))?.len();
    let total_other = db.read_rows(-1, Some(Other))?.len();

    let checkin_percent = percent(total_checkins, total_notes);
    let todo_percent = percent(total_todos, total_notes);
    let idea_percent = percent(total_ideas, total_notes);
    let other_percent = percent(total_other, total_notes);

    let count_width = total_notes.to_string().len();

    Ok(format!(
        "\
Monochromium Stats

  {:<15} {:>width$}
  {:<15} {:>width$} {:>5.0}%
  {:<15} {:>width$} {:>5.0}%
  {:<15} {:>width$} {:>5.0}%
  {:<15} {:>width$} {:>5.0}%
",
        "Total notes",
        total_notes,
        "Check-ins",
        total_checkins,
        checkin_percent,
        "Todos",
        total_todos,
        todo_percent,
        "Ideas",
        total_ideas,
        idea_percent,
        "Other",
        total_other,
        other_percent,
        width = count_width
    ))
}

fn percent(part: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 / total as f64 * 100.0
    }
}
