use crate::config::Config;
use crate::database::Database;
use crate::storage::read_file;
use crate::types::{MonoError, NoteTypes};
use chrono::{Duration, Local};
use std::path::PathBuf;

pub fn reminder(db: &Database, config: Config) -> Result<String, MonoError> {
    let current_datetime = Local::now().naive_local();
    let interval = config.checkin_interval_minutes;

    let rows = db.search_rows(None, 1, Some(NoteTypes::CheckIn))?;
    let checkins = !rows.is_empty();

    let default = get_default(checkins);

    let todos: String = if config.show_todo_list {
        get_todo_list(db)?
    } else {
        String::new()
    };

    let final_out: String = [default, todos.clone()].join("\n");

    if rows.is_empty() {
        return Ok(final_out);
    }

    let past_datetime = rows[0].date;
    let time_passed: Duration = current_datetime - past_datetime;

    if time_passed.num_minutes() >= interval {
        Ok(final_out)
    } else {
        if config.show_todo_list {
            Ok(todos.trim().to_string())
        } else {
            Ok(String::new())
        }
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

fn get_todo_list(db: &Database) -> Result<String, MonoError> {
    let mut notes = db.read_rows(-1, Some(NoteTypes::Todo))?;

    for note in &mut notes {
        note.content = Some(read_file(&PathBuf::from(&note.file_link))?);
    }

    let mut pre: Vec<String> = vec!["".to_string(), "-- todo --".to_string()];
    for note in notes {
        pre.push(format!("[{}] {}", note.id, note.content.unwrap()));
    }

    Ok(pre.join("\n"))
}
