use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    // pub fn add_minutes(&self, minutes: i32) -> Self {
    //     // todo!("Add {minutes} minutes to existing Clock time");
    // }

    // todo!("utilise .to_string() directily");
    // default fn to_string(&self) -> String {
    //
    // }
}

// Using Display
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Here we define how the struct should be formatted as a string
        write!(f, "{}, {}", self.hours, self.minutes)
    }
}