#[derive(Debug, Default)]
pub enum WatchMode {
    #[default]
    Timekeeping,
    Alarm,
    Stopwatch,
    DualTime,
}

impl WatchMode {
    pub fn prefix(&self) -> &'static str {
        match self {
            WatchMode::Timekeeping => "TM",
            WatchMode::Alarm => "AL",
            WatchMode::Stopwatch => "ST",
            WatchMode::DualTime => "DT",
        }
    }
}

impl PartialEq for WatchMode {
    fn eq(&self, other: &Self) -> bool {
        self.prefix() == other.prefix()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::modes::WatchMode;

    #[test]
    fn test_prefix() {
        let alarm = WatchMode::Alarm;
        let dual = WatchMode::DualTime;
        let time = WatchMode::Timekeeping;
        let stopwatch = WatchMode::Stopwatch;

        assert_eq!(alarm.prefix(), "AL");
        assert_eq!(dual.prefix(), "DT");
        assert_eq!(time.prefix(), "TM");
        assert_eq!(stopwatch.prefix(), "ST");
    }

    #[test]
    fn test_eq() {
        let alarm_1 = WatchMode::Alarm;
        let alarm_2 = WatchMode::Alarm;
        let dual_1 = WatchMode::DualTime;
        let dual_2 = WatchMode::DualTime;

        assert!(alarm_1 == alarm_2);
        assert!(dual_1 == dual_2);
        assert!(alarm_1 != dual_1);
    }
}
