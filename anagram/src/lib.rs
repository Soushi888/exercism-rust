use std::collections::HashSet;

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

pub fn is_anagram(word_1: &str, word_2: &str) -> bool {


    todo!()
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // possible_anagrams.contains(&word);
    let mut output = HashSet::new();


    let word_letters: Vec<char> = word.chars().collect();
    println!("{:?}", word_letters);

    for i in 0..possible_anagrams.len() {
        let mut word_letters: Vec<char> = word.chars().collect();
        let possible_anagram = possible_anagrams[i];
        let possible_anagram_letters: Vec<char> = possible_anagram.chars().collect();

        println!("{}, {}", possible_anagram.len(), word.len());
        println!("{:?}", possible_anagram_letters);

        if is_anagram(word, possible_anagram) {
            output.insert(possible_anagram);
        }
    }

    output
}
