use chrono::{DateTime, Local};

// %A = Full weekday name (e.g., Monday)
// %H:%M:%S = Hours, minutes, and seconds
const FMT_24H: &str = "%A, %H:%M:%S";
const FMT_12H: &str = "%A, %I:%M:%S %p";

pub fn local_datetime(hour_format: i8) -> String {
    // Get and format local datetime
    format_datetime(Local::now(), hour_format)
}

pub fn format_datetime(datetime: DateTime<Local>, hour_format: i8) -> String {
    // 1. Check user format
    let mut user_format = FMT_24H;
    if hour_format == 12 {
        user_format = FMT_12H;
    }

    // 2. Format and return
    datetime.format(user_format).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_local_datetime() {
        // Create a frozen, predictable mock datetime (Monday, Oct 26, 2026 at 14:00:00)
        let mock_time = Local.with_ymd_and_hms(2026, 10, 26, 14, 0, 0).unwrap();

        let datetime_12h = format_datetime(mock_time, 12);
        assert!(datetime_12h.contains("Monday"));
        assert!(datetime_12h.contains("Monday, 02:00:00"));
        assert!(datetime_12h.contains("PM"));
        assert_eq!(datetime_12h, "Monday, 02:00:00 PM");

        let datetime_24h = format_datetime(mock_time, 24);
        assert!(datetime_24h.contains("Monday"));
        assert!(datetime_24h.contains("Monday, 14:00:00"));
        assert!(!datetime_24h.contains("PM"));
        assert_eq!(datetime_24h, "Monday, 14:00:00");
    }
}
