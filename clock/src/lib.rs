use std::fmt;

const MINUTES_IN_DAY: i64 = 24 * 60;
const MINUTES_IN_HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        let input_in_minutes = hours * MINUTES_IN_HOUR + minutes;
        Self {
            minutes: ((input_in_minutes % MINUTES_IN_DAY) + MINUTES_IN_DAY) % MINUTES_IN_DAY
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / MINUTES_IN_HOUR, self.minutes % MINUTES_IN_HOUR)
    }
}