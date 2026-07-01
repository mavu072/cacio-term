use crate::audio::beep::system_beep;

use chrono::{Local, NaiveDate, NaiveTime, Timelike};

#[derive(Debug, Default)]
pub struct Alarm {
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    has_triggered: bool,
}

impl Alarm {
    fn set_triggered_false(&mut self) {
        self.has_triggered = false;
    }

    pub fn set_time(&mut self, hour: u32, min: u32, sec: u32) -> bool {
        if let Some(valid_time) = NaiveTime::from_hms_opt(hour, min, sec) {
            self.time = Some(valid_time);
            self.set_triggered_false();
            true
        } else {
            false
        }
    }

    pub fn set_date(&mut self, year: i32, month: u32, day: u32) -> bool {
        if let Some(valid_date) = NaiveDate::from_ymd_opt(year, month, day) {
            self.date = Some(valid_date);
            self.set_triggered_false();
            true
        } else {
            false
        }
    }

    pub fn trigger(&mut self) {
        // 1. Check time exists
        let conf_time = match self.time {
            Some(t) => t,
            None => return,
        };

        let now = Local::now();
        let current_date = now.date_naive();
        let current_time = now.time();

        // 2. Check Date
        if let Some(conf_date) = self.date {
            if conf_date != current_date {
                return;
            }
        }

        // 3. Check Time
        let alarm_seconds = conf_time.num_seconds_from_midnight();
        let current_seconds = current_time.num_seconds_from_midnight();

        if alarm_seconds == current_seconds {
            // Sound alarm
            if !self.has_triggered {
                system_beep();
                self.has_triggered = true; // Flag prevent sound the alarm everytime during the second.
            }
        } else {
            // Reset flag once second passes.
            self.set_triggered_false();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Duration, Timelike};

    #[test]
    fn test_set_time() {
        let mut alarm = Alarm::default();
        assert!(alarm.set_time(10, 30, 15));

        let configured_time = alarm.time.unwrap();
        assert_eq!(configured_time.hour(), 10);
        assert_eq!(configured_time.minute(), 30);
        assert_eq!(configured_time.second(), 15);
    }

    #[test]
    fn test_set_date() {
        let mut alarm = Alarm::default();
        assert!(alarm.set_date(2026, 07, 01));

        let configured_time = alarm.date.unwrap();
        assert_eq!(configured_time.year(), 2026);
        assert_eq!(configured_time.month(), 7);
        assert_eq!(configured_time.day(), 1);
    }

    #[test]
    fn test_invalid_inputs() {
        let mut alarm = Alarm::default();
        assert!(!alarm.set_time(25, 0, 0)); // Invalid hour
        assert!(!alarm.set_date(2026, 2, 30)); // Invalid date
    }

    #[test]
    fn test_trigger() {
        let mut alarm = Alarm::default();
        let now = Local::now();

        assert!(alarm.set_time(now.hour(), now.minute(), now.second()));
        alarm.trigger();
        assert!(alarm.has_triggered);
    }

    #[test]
    fn test_not_trigger() {
        let mut alarm = Alarm::default();
        let now = Local::now() + Duration::hours(2); // Add 2 hours

        assert!(alarm.set_time(now.hour(), now.minute(), now.second()));
        alarm.trigger();
        assert!(!alarm.has_triggered);
    }
}
