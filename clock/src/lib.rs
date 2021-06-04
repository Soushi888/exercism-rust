use std::fmt::{Display, Formatter, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
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

        match self.hours {
            24 => hours = String::from("00"),
            mut h if h > 24 => {
                let cycles_number = h / 24;
                h = h - (24 * cycles_number);

                if h < 10 { hours = format!("0{}", h) } else { hours = h.to_string() }
            }
            h => {
                if h < 10 {
                    hours = format!("0{}", h)
                } else {
                    hours = self.hours.to_string()
                }
            }
        }

        match self.minutes {
            mut m if m > 60 => {
                let cycles_number = m / 60;

                match i32::from_str(&*hours) {
                    Ok(h) => hours = (h + 1).to_string(),
                    Err(e) => ()
                }
                m = m - (60 * cycles_number);

                if m < 10 { minutes = format!("0{}", self.minutes) }
            }
            m => {
                if m < 10 {
                    minutes = format!("0{}", m)
                } else {
                    minutes = self.minutes.to_string()
                }
            }
        }

        write!(f, "{}:{}", hours, minutes)
    }
}
