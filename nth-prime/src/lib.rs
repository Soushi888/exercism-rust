/// Return true if n: u32 is a prime number.
fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }

    for i in 2..n {
        if n % i == 0 { return false; }
    }

    return true;
}

/// Return the prime number of the primes numbers suite corresponding to the n: u32 index.
pub fn nth(n: u32) -> u32 {
    let mut primes_vec: Vec<u32> = vec![2, 3];

    for i in 1..n {
        let mut j: u32 = primes_vec[i as usize] + 1;
        while !is_prime(j) {
            j += 1;
        }
        j += 1;

        primes_vec.push(j - 1)
    }

    let mut indexed_prime_vec = primes_vec[(n) as usize];
    if n >= 2 { indexed_prime_vec = primes_vec[(n) as usize] };

    indexed_prime_vec
}
