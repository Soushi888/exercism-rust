// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const LOWEST_CARS_PER_HOUR: u16 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let base_production = (LOWEST_CARS_PER_HOUR * speed as u16) as f64;
    let cars_produced: f64 = match speed {
        0..=4 => base_production,
        5..=8 => base_production * 0.9,
        9..=10 => base_production * 0.77,
        _ => panic!("Speed must be a number between 0 and 10.")
    };

    cars_produced
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
