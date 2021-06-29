use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MAX_MINUTES: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (hours * 60 + minutes).rem_euclid(MAX_MINUTES);

        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, minutes + self.minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
