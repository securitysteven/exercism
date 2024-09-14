// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0; // Seconds in one Earth year

#[derive(Debug)]
pub struct Duration {
    seconds: i32
}

impl Duration {
    fn from(seconds: i32) -> Self {
        Duration { seconds }
    }
     fn in_seconds(&self) -> u64 {
        self.seconds
    }
}

pub trait Planet {
    fn years_during(duration: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;


impl Planet for Mercury {
    fn years_during(duration: &Duration) -> f64 {
        duration.seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 0.2408467)
    }
}

impl Planet for Venus {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 0.61519726)
    }
}

impl Planet for Earth {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / EARTH_YEAR_IN_SECONDS
    }
}

impl Planet for Mars {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 1.8808158)
    }
}

impl Planet for Jupiter {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 11.862615)
    }
}

impl Planet for Saturn {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 29.447498)
    }
}

impl Planet for Uranus {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 84.016846)
    }
}

impl Planet for Neptune {
    fn years_during(duration: &Duration) -> f64 {
        duration.in_seconds() as f64 / (EARTH_YEAR_IN_SECONDS * 164.79132)
    }
}