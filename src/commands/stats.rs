use crate::database::Database;
use crate::types::MonoError;
use std::collections::HashMap;

pub fn get_stats(db: &Database) -> Result<String, MonoError> {
    let counts: HashMap<String, u32> = db.count_rows()?.into_iter().collect();

    let total_checkins = counts.get("check in").copied().unwrap_or(0);
    let total_todos = counts.get("todo").copied().unwrap_or(0);
    let total_ideas = counts.get("idea").copied().unwrap_or(0);
    let total_other = counts.get("other").copied().unwrap_or(0);
    let total_notes = total_checkins + total_todos + total_ideas + total_other;

    let checkin_percent = percent(total_checkins, total_notes);
    let todo_percent = percent(total_todos, total_notes);
    let idea_percent = percent(total_ideas, total_notes);
    let other_percent = percent(total_other, total_notes);

    let count_width = total_notes.to_string().len();

    Ok(format!(
        "\
Monochromium Stats (Current Notes)

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

fn percent(part: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 / total as f64 * 100.0
    }
}
