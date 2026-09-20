use crate::config::Config;
use crate::database::Database;
use crate::types::NoteTypes;
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn reminder(db: &Database, config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let current_datetime = Local::now().naive_local();
    let interval = config.checkin_interval_minutes;

    let rows = db.search_notes(None, 1, Some(NoteTypes::CheckIn))?;
    if rows.is_empty() {
        return Ok([
            "-- monochromium --",
            "No check-ins yet.",
            "Start one with `mono checkin \"text\"`.",
        ]
        .join("\n"));
    }

    let past_datetime = rows[0].date;
    let time_passed: Duration = current_datetime - past_datetime;

    if time_passed.num_minutes() >= interval {
        Ok([
            "-- monochromium --",
            "It's been a while since your last check-in.",
            "Write one with `mono checkin \"text\"`.",
        ]
        .join("\n"))
    } else {
        Ok(String::new())
    }
}
