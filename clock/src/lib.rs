use std::fmt::{Display, Formatter, Result};

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let minutes_cycles_number = minutes / MINUTES_PER_HOUR;
        minutes = minutes - (MINUTES_PER_HOUR * minutes_cycles_number);

        hours += minutes_cycles_number;
        let hours_cycles_number = hours / HOURS_PER_DAY;
        hours = hours - (HOURS_PER_DAY * hours_cycles_number);

        if hours >= HOURS_PER_DAY {
            hours = 0 + (minutes_cycles_number - (if hours_cycles_number > 0 {
                HOURS_PER_DAY + hours_cycles_number
            } else {
                HOURS_PER_DAY
            }));
        };

        if minutes < 0 {
            minutes += MINUTES_PER_HOUR;
            hours -= 1;
        }

        if hours < 0 {
            hours += HOURS_PER_DAY;
        }

        Clock {
            hours,
            minutes,
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.minutes += minutes;

        Clock::new(self.hours, self.minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
