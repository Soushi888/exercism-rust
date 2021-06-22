use std::collections::HashSet;

/// Return true if `n: u32` is a prime number.
fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }

    for i in 2..n {
        if n % i == 0 { return false; }
    }

    return true;
}

/// Return the prime number of the primes numbers suite corresponding to the `n: u32` index.
pub fn prime_nth(n: u32) -> u32 {
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

/// Return the sum of all the primes numbers corresponding to the alphabetical index of the chars of a word.
pub fn alphabetic_primes_sum(word: &str) -> u32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut alphabet_primes_numbers: Vec<u32> = vec![];
    for i in 0..alphabet.len() {
        alphabet_primes_numbers.push(prime_nth(i as u32));
    }

    let mut word_letters: Vec<char> = word.chars().collect();
    let mut primes_sum: u32 = 0;

    for i in 0..word.len() {
        for j in 0..alphabet.len() {
            if word_letters[i] == alphabet[j] {
                primes_sum += alphabet_primes_numbers[j];
            }
        }
    }

    primes_sum
}

/// Return true if word_1 and word_2 are anagrams.
pub fn is_anagram(word_1: &str, word_2: &str) -> bool {
    alphabetic_primes_sum(word_1) == alphabetic_primes_sum(word_2)
}

/// Return an HashSet of anagrams among a dictionary array for a specific word.
pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();

    for i in 0..possible_anagrams.len() {
        let possible_anagram = possible_anagrams[i];

        if is_anagram(word, possible_anagram) {
            output.insert(possible_anagram);
        }
    }

    output
}
