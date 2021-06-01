pub fn is_leap_year(year: u64) -> bool {
    match (year % 4 == 0, year % 100 == 0, year % 400 == 0) {
        (true, true, true) => true,
        (true, false, false) => true,
        (true, true, false) => false,
        _ => false
    }
}
