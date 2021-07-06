pub fn is_armstrong_number(num: u32) -> bool {
    let figures: Vec<u32> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let mut total: u32 = 0;

    for i in &figures {
        total += i.pow(figures.len() as u32);
    }

    total == num
}
