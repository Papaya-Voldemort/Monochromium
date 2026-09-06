use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};

pub async fn reminder(conn: libsql::Connection) {
    //     Get current time compare against last checkin
    let now: DateTime<Local> = Local::now();

    let current_time: NaiveTime = now.time();
    let current_date: NaiveDate = now.date_naive();

    let current_datetime: NaiveDateTime = current_date.and_time(current_time);

}
