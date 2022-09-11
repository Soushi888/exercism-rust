/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");
    let is_digits = code.chars().all(|c| c.is_ascii_digit());
    if code.len() <= 1 || !is_digits { return false; }

    code.chars().rev().enumerate().fold(0, |sum, (i, c)| {
        let mut n = c.to_digit(10).unwrap();
        if i % 2 == 1 {
            n *= 2;
            if n > 9 { n -= 9; }
        }
        sum + n
    }) % 10 == 0
}
