use crate::commands::definitions::NoteTypes::CheckIn;
use crate::database::search_notes;
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub async fn reminder(conn: libsql::Connection) -> Result<String, Box<dyn std::error::Error>> {
    //     Get current time compare against last checkin
    let now: DateTime<Local> = Local::now();

    let current_time: NaiveTime = now.time();
    let current_date: NaiveDate = now.date_naive();

    let current_datetime: NaiveDateTime = current_date.and_time(current_time);

    let rows = search_notes(conn, None, 1, Some(CheckIn)).await?;
    if rows.is_empty() {
        return Ok(
            [
                "Hey, it looks like you have never checked in!",
                "Try running `mono checkin \"text\"` to make one!",
                "A check-in is just like a regular note, but we'll remind you to make one every once in a while!",
                "You can use these any way you want, but we recommend them as a mental health check-in.",
            ]
                .join("\n"),
        );
    }

    let past_note = &rows[0];

    let format: &str = "%Y-%m-%d %H:%M:%S%.f";
    let past_date_raw = past_note.date.trim();

    let past_datetime = match NaiveDateTime::parse_from_str(past_date_raw, format) {
        Ok(datetime) => datetime,
        Err(e) => return Err(Box::new(e)),
    };

    let time_passed: Duration = current_datetime - past_datetime;

    let hours = time_passed.num_hours();

    if hours >= 24 {
        Ok(
            "It's been a bit since your last check-in. Write how you're doing with `mono checkin \"text\"`"
                .to_string(),
        )
    } else {
        Ok(String::new())
    }
}
