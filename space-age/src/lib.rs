// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 0.61519726;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 1.0;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 1.8808158;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 11.862615;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 29.447498;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 84.016846;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let speed: f64 = 164.79132;
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.25 / speed) * 100.0) / 100.0
    }
}