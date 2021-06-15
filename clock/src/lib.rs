use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;
use std::cmp::min;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let minutes_cycles_number = minutes / 60;
        minutes = minutes - (60 * minutes_cycles_number);

        hours += minutes_cycles_number;
        let hours_cycles_number = hours / 24;
        hours = hours - (24 * hours_cycles_number);

        if hours >= 24 {
            hours = 0 + (minutes_cycles_number - (if hours_cycles_number > 0 {
                24 + hours_cycles_number
            } else {
                24
            }));
        };

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24;
        }

        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        println!("{} {} minutes", self, minutes);

        self.minutes += minutes;

        if self.minutes >= 60 {
            let minutes_cycles_number = self.minutes / 60;
            self.hours += minutes_cycles_number;
            self.minutes -= 60 * minutes_cycles_number;
        }

        let hours_cycles_number = self.hours / 24;
        if self.hours >= 24 {
            self.hours -= 24 * hours_cycles_number;
        };


        if self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }

        if self.hours < 0 {
            self.hours += 24;
        }

        Clock {
            hours: self.hours,
            minutes: self.minutes,
        }
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
