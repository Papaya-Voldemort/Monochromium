use crate::config::Config;
use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use chrono::{Duration, Local};

pub fn reminder(db: &Database, config: Config) -> Result<String, MonoError> {
    let current_datetime = Local::now().naive_local();
    let interval = config.checkin_interval_minutes;

    let default = [
        "-- monochromium --",
        "No check-ins yet.",
        "Start one with `mono checkin \"text\"`.",
    ]
    .join("\n");

    let final_out: String = if config.show_todo_list {
        let notes = db.read_rows(-1, Some(NoteTypes::Todo))?;
        let mut pre: Vec<String> = vec![default, "".to_string(), "-- todo --".to_string()];
        for note in notes {
            pre.push(format!("[{}] {}", note.id, note.content));
        }

        pre.join("\n")
    } else {
        default
    };

    let rows = db.search_rows(None, 1, Some(NoteTypes::CheckIn))?;
    if rows.is_empty() {
        return Ok(final_out);
    }

    let past_datetime = rows[0].date;
    let time_passed: Duration = current_datetime - past_datetime;

    if time_passed.num_minutes() >= interval {
        Ok(final_out)
    } else {
        Ok(String::new())
    }
}
