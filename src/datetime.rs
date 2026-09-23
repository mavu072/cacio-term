use chrono::{DateTime, Local};

// %H:%M:%S = Hours, minutes, and seconds
const DATE_FMT_24H: &str = "%H:%M:%S";
const DATE_FMT_12H: &str = "%I:%M:%S %p";

pub fn local_datetime(hour_format: i8) -> (String, String, String, String) {
    // Get and format local time

    let datetime_now = Local::now();

    // Return tuple
    (
        fmt_time(datetime_now, hour_format),
        fmt_day(datetime_now),
        fmt_year(datetime_now),
        fmt_datemonth(datetime_now),
    )
}

pub fn fmt_time(datetime: DateTime<Local>, hour_format: i8) -> String {
    // 1. Check user format
    let mut user_format = DATE_FMT_24H;
    if hour_format == 12 {
        user_format = DATE_FMT_12H;
    }

    // 2. Format and return
    datetime.format(user_format).to_string()
}

pub fn fmt_day(datetime: DateTime<Local>) -> String {
    // %A = Full weekday name (e.g., Monday)
    datetime.format("%A").to_string()
}

pub fn fmt_year(datetime: DateTime<Local>) -> String {
    datetime.format("%Y").to_string()
}

pub fn fmt_datemonth(datetime: DateTime<Local>) -> String {
    // %-m = Month number (not zero-padded)
    // %d = Day of the month (zero-padded)
    datetime.format("%-m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_local_datetime() {
        // Create a frozen, predictable mock datetime (Monday, Oct 26, 2026 at 14:00:00)
        let mock_time = Local.with_ymd_and_hms(2026, 10, 26, 14, 0, 0).unwrap();

        let time_12h = fmt_time(mock_time, 12);
        assert_eq!(time_12h, "02:00:00 PM");

        let time_24h = fmt_time(mock_time, 24);
        assert_eq!(time_24h, "14:00:00");

        let day = fmt_day(mock_time);
        assert_eq!(day, "Monday");

        let year = fmt_year(mock_time);
        assert_eq!(year, "2026");

        let datemonth = fmt_datemonth(mock_time);
        assert_eq!(datemonth, "10-26");
    }
}
