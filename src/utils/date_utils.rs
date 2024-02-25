use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

pub fn parse_date_time_with_timezone(date_str: &str, timezone_offset: i32) -> DateTime<Utc> {
    let naive_date_time = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse date and time");

    // Convert timezone offset to seconds and explicitly convert it to i64
    let timezone_offset_seconds: i64 = (timezone_offset * 3600) as i64;

    // Adjust the naive date time with timezone offset
    let adjusted_date_time = naive_date_time.timestamp() - timezone_offset_seconds;

    // Create DateTime<Utc> object
    Utc.timestamp_opt(adjusted_date_time, 0)
        .single()
        .expect("Failed to create DateTime<Utc>")
}
