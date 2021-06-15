pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit <= 1 { return 0; }
    println!("factors = {:?}, limit = {}", factors, limit);
    println!();

    let mut current_multiple: u32 = factors[0];
    let mut sum: u32 = 0;

    while current_multiple < limit {
        for i in 0..factors.len() {
            if current_multiple % factors[i] == 0 {
                sum += current_multiple;
                println!("i = {}, sum = {}", i, sum);
            }
        }
        println!("current multiple = {}, sum = {}", current_multiple, sum);
        println!();

        current_multiple += 1;
    }

    sum
}
