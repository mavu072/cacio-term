use crate::audio::beep::system_beep;

use chrono::{Local, NaiveDate, NaiveTime, Timelike};

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Alarm {
    time: Option<NaiveTime>,
    date: Option<NaiveDate>,
    has_triggered: bool,
    active_alarm_types: Vec<AlarmType>,
}

#[allow(dead_code)]
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

    pub fn add_active_type(&mut self, at: AlarmType) -> bool {
        // Dedup
        if self.active_alarm_types.contains(&at) {
            return false;
        }

        // Maximum 3 types
        if self.active_alarm_types.len() < 3 {
            self.active_alarm_types.push(at);
            true
        } else {
            false
        }
    }

    pub fn remove_active_type(&mut self, al_type: AlarmType) -> bool {
        if let Some(index) = self.active_alarm_types.iter().position(|&at| at == al_type) {
            self.active_alarm_types.remove(index);
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlarmType {
    Snooze,
    Alarm,
    Signal,
}

impl AlarmType {
    const ALL: [AlarmType; 3] = [AlarmType::Snooze, AlarmType::Alarm, AlarmType::Signal];

    pub fn prefix(&self) -> &'static str {
        match self {
            AlarmType::Snooze => "SNZ",
            AlarmType::Alarm => "ALM",
            AlarmType::Signal => "SIG",
        }
    }

    // pub fn as_list() -> Vec<AlarmType> {
    //     Self::ALL.to_vec()
    // }

    pub fn as_str_list() -> Vec<&'static str> {
        Self::ALL.iter().map(|&alarm| alarm.prefix()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

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
    fn test_add_active_type() {
        let mut alarm = Alarm::default();
        let signal = AlarmType::Signal;
        assert!(alarm.add_active_type(signal));
        assert!(alarm.active_alarm_types.len() == 1);
    }

    #[test]
    fn test_add_active_type_dedup() {
        let mut alarm = Alarm::default();
        assert!(alarm.add_active_type(AlarmType::Signal));
        assert!(!alarm.add_active_type(AlarmType::Signal));
        assert!(!alarm.add_active_type(AlarmType::Signal));
        assert!(alarm.active_alarm_types.len() == 1);
    }

    #[test]
    fn test_add_active_type_max_reached() {
        let mut alarm = Alarm::default();
        assert!(alarm.add_active_type(AlarmType::Alarm));
        assert!(alarm.add_active_type(AlarmType::Signal));
        assert!(alarm.add_active_type(AlarmType::Snooze));
        assert!(!alarm.add_active_type(AlarmType::Signal));

        assert!(alarm.active_alarm_types.len() == 3);
    }

    #[test]
    fn test_remove_active_type() {
        let mut alarm = Alarm::default();
        let snooze = AlarmType::Snooze;
        alarm.add_active_type(snooze);

        assert!(alarm.active_alarm_types.len() == 1);
        assert!(alarm.remove_active_type(AlarmType::Snooze));
        assert!(alarm.active_alarm_types.len() == 0);
    }

    #[test]
    fn test_remove_active_type_preserves_other() {
        let mut alarm = Alarm::default();
        alarm.add_active_type(AlarmType::Alarm);
        alarm.add_active_type(AlarmType::Signal);
        alarm.add_active_type(AlarmType::Snooze);

        assert!(alarm.active_alarm_types.len() == 3);
        assert!(alarm.remove_active_type(AlarmType::Signal));
        assert!(alarm.active_alarm_types.len() == 2);
    }

    #[test]
    fn test_remove_active_type_not_found() {
        let mut alarm = Alarm::default();

        assert!(alarm.active_alarm_types.len() == 0);
        assert!(!alarm.remove_active_type(AlarmType::Snooze));
        assert!(alarm.active_alarm_types.len() == 0);
    }
}

#[cfg(test)]
mod trigger_tests {
    use super::*;
    use chrono::Duration;

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

#[cfg(test)]
mod alarm_type_tests {
    use super::*;

    #[test]
    fn test_eq() {
        let alarm_1 = AlarmType::Alarm;
        let alarm_2 = AlarmType::Alarm;
        let signal_1 = AlarmType::Signal;
        let signal_2 = AlarmType::Signal;
        let snooze_1 = AlarmType::Snooze;
        let snooze_2 = AlarmType::Snooze;

        assert!(alarm_1 == alarm_2);
        assert!(signal_1 == signal_2);
        assert!(snooze_1 == snooze_2);

        assert!(alarm_1 != signal_1);
        assert!(signal_1 != snooze_1);
        assert!(alarm_1 != snooze_1);
    }
}
