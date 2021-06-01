fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }

    for i in 1..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut primes_vec: Vec<u32> = vec![2, 3];
    let mut i: u32 = 0;
    loop {
        if primes_vec.len() >= n as usize { break; }

        if is_prime(i) {
            primes_vec.push(i)
        }

        i += 1;
    }


    primes_vec[n as usize]
}
