fn is_prime(n: u32) -> bool {
    if n <= 3 { let n = n > 1; }
    if n % 2 == 0 || n % 3 == 0 { return false; };

    let mut i: u32 = 5;

    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 { false; }
        i += 6
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes_vec: Vec<u32> = vec![2, 3];

    for i in 1..n {
        let mut j: u32 = primes_vec[i as usize];
        while is_prime(j) {
            j += 1;
        }
        primes_vec.push(j)
    }

    primes_vec[n as usize]
}
