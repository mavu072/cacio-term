#[derive(Debug, Default)]
pub enum WatchMode {
    #[default]
    Timekeeping,
    Alarm,
    Stopwatch,
    DualTime,
}

impl WatchMode {
    fn prefix(&self) -> &'static str {
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
