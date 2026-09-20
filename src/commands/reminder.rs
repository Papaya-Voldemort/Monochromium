use crate::config::Config;
use crate::database::Database;
use crate::types::NoteTypes;
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub fn reminder(db: &Database, config: Config) -> Result<String, Box<dyn std::error::Error>> {
    // Get current time compare against last checkin
    let now: DateTime<Local> = Local::now();
    let interval = config.checkin_interval_minutes;

    let current_time: NaiveTime = now.time();
    let current_date: NaiveDate = now.date_naive();

    let current_datetime: NaiveDateTime = current_date.and_time(current_time);

    let rows = db.search_notes(None, 1, Some(NoteTypes::CheckIn))?;
    if rows.is_empty() {
        return Ok([
            "-- monochromium --",
            "No check-ins yet.",
            "Start one with `mono checkin \"text\"`.",
        ]
        .join("\n"));
    }

    let past_note = &rows[0];

    let format: &str = "%Y-%m-%d %H:%M:%S%.f";
    let past_date_raw = past_note.date.trim();

    let past_datetime = match NaiveDateTime::parse_from_str(past_date_raw, format) {
        Ok(datetime) => datetime,
        Err(e) => return Err(Box::new(e)),
    };

    let time_passed: Duration = current_datetime - past_datetime;

    let mins = time_passed.num_minutes();

    if mins >= interval {
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
