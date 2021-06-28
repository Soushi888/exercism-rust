const SECONDS_PER_YEARS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
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
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 0.2408467;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 0.61519726;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 1.0;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 1.8808158;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 11.862615;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 29.447498;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 84.016846;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 164.79132;
        d.0 as f64 / SECONDS_PER_YEARS / speed
    }
}
