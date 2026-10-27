use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use crate::utils::{MOOD_SPAN, fetch_mood_days, mood_footer, mood_table};
use chrono::{Duration, Local, NaiveDate};
use std::collections::HashMap;

pub fn get_stats(db: &Database, short: bool, colorize: bool) -> Result<String, MonoError> {
    let today: NaiveDate = Local::now().date_naive();
    let window_start = today - Duration::days(MOOD_SPAN as i64 - 1);

    let checkins = db.read_rows(-1, Some(NoteTypes::CheckIn), Some(window_start), None)?;
    let mood = fetch_mood_days(today, checkins);

    let counts: HashMap<String, u32> = db.count_rows(None)?.into_iter().collect();

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

    let mut out = format!(
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
    );

    if mood.iter().any(|day| !day.scores.is_empty()) {
        let view = if short {
            mood_footer(&mood, colorize)
        } else {
            mood_table(&mood, colorize)
        };
        out.push('\n');
        out.push_str(&view);
    }
    Ok(out)
}

fn percent(part: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        part as f64 / total as f64 * 100.0
    }
}
