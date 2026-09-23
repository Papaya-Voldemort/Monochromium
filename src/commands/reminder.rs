use crate::config::Config;
use crate::database::Database;
use crate::types::{MonoError, NoteTypes};
use chrono::{Duration, Local};

pub fn reminder(db: &Database, config: Config) -> Result<String, MonoError> {
    let current_datetime = Local::now().naive_local();
    let interval = config.checkin_interval_minutes;
    let mut checkins: bool = true;

    let rows = db.search_rows(None, 1, Some(NoteTypes::CheckIn))?;
    if rows.is_empty() {
        checkins = false;
    }

    let default = get_default(checkins);

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

    let past_datetime = rows[0].date;
    let time_passed: Duration = current_datetime - past_datetime;

    if time_passed.num_minutes() >= interval {
        Ok(final_out)
    } else {
        Ok(String::new())
    }
}

fn get_default(checkins: bool) -> String {
    let body = if checkins {
        "It's been a while since your last check-in.\nYou can make one with `mono add --type check-in \"text\"`."
    } else {
        "No check-ins yet.\nStart one with `mono add --type check-in \"text\"`."
    };

    body.to_string()
}
