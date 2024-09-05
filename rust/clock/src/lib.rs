use std::fmt;

pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hour: hours, minute: minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        self.minute += minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Here we define how the struct should be formatted as a string
        write!(f, "{}:{}", self.hour, self.minute)
    }
}