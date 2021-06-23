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
    fn years_during(d: &Duration) -> f64 {
        f64::trunc((d.0 as f64 / 60.0 / 60.0 / 24.0 / 365.0) * 100.0) / 100.0
    }
}

pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {}

impl Planet for Venus {}

impl Planet for Earth {}

impl Planet for Mars {}

impl Planet for Jupiter {}

impl Planet for Saturn {}

impl Planet for Uranus {}

impl Planet for Neptune {}
