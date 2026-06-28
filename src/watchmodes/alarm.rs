use crate::audio::beep::system_beep;

use chrono::{DateTime, Datelike, Local};

#[derive(Debug, Default)]
pub struct Alarm {
    alarm_time: DateTime<Local>,
    alarm_date: Option<DateTime<Local>>,
}

impl Alarm {
    fn set_alarm_time(&mut self) {
        todo!();
    }

    fn trigger_alarm(&mut self) {
        let datetime_now = Local::now();
        let mut is_alarm_date = false;

        match self.alarm_date {
            Some(alm_date) => {
                if alm_date.month() == datetime_now.month() && alm_date.day() == datetime_now.day()
                {
                    is_alarm_date = true;
                }
            } // check date
            None => {} // No-op
        }

        if is_alarm_date {
            // Sound alarm
            system_beep();
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_set_alarm_time() {
        todo!();
        assert!(true);
    }
}
