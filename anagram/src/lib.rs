use std::collections::HashSet;

/// Return true if word_1 and word_2 are anagrams.
pub fn is_anagram(word_1: &str, word_2: &str) -> bool {
    let mut word_1_letters: Vec<char> = word_1.to_lowercase().chars().collect();
    word_1_letters.sort();

    let mut word_2_letters: Vec<char> = word_2.to_lowercase().chars().collect();
    word_2_letters.sort();

    if word_1.to_lowercase() != word_2.to_lowercase() {
        word_1_letters == word_2_letters
    } else {
        false
    }
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
