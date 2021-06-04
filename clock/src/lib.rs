use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let hours_cycles_number = hours / 24;
        let minutes_cycles_number = minutes / 60;

        hours = hours - (24 * hours_cycles_number);
        minutes = minutes - (60 * minutes_cycles_number);

        hours += minutes_cycles_number;
        if hours >= 24 { hours = 0 + minutes_cycles_number };

        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut hours: String = "".to_owned();
        let mut minutes: String = "".to_owned();

        if self.hours < 10 {
            hours = format!("0{}", self.hours)
        } else {
            hours = self.hours.to_string()
        }

        if self.minutes < 10 {
            minutes = format!("0{}", self.minutes)
        } else {
            minutes = self.minutes.to_string()
        }

        write!(f, "{}:{}", hours, minutes)
    }
}
