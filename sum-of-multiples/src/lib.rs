pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit <= 1 { return 0; }
    if factors == [] { return 0; }

    let mut current_multiple: u32 = factors[0];
    let mut sum: u32 = 0;
    let mut is_counted: bool;

    while current_multiple < limit {
        is_counted = false;

        for i in 0..factors.len() {
            if factors[i] != 0 {
                if current_multiple % factors[i] == 0 && !is_counted {
                    sum += current_multiple;

                    is_counted = true;
                }
            }
        }

        current_multiple += 1;
    }

    sum
}
