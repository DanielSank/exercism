use std::fmt;
use std::cmp::{PartialEq, Eq};


fn resolve(hours: i32, minutes: i32) -> (i32, i32) {
    let mut minutes = 60 * hours + minutes;
    while minutes < 0 {
        minutes = minutes +  60 * 24;
    }
    // Now minutes is positive
    while minutes >= 60 * 24 {
        minutes = minutes - (60 * 24);
    }
    // Now we have less than one day
    let hours = minutes / 60;
    let minutes = minutes - hours * 60;
    (hours, minutes)
}

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = resolve(hours, minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = resolve(self.hours, self.minutes + minutes);
        Clock { minutes, hours }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
