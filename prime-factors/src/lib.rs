/// Return a vector of the factors of a given unsigned integer
pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut i: u64 = 2;
    let mut current_factor: u64 = n;

    while i.pow(2) <= current_factor {
        while current_factor % i == 0 {
            prime_factors.push(i);
            current_factor /= i;
        }

        i += 1;
    }

    if current_factor != 1 {
        prime_factors.push(current_factor)
    }

    prime_factors
}