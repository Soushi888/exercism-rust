use std::collections::HashSet;

/// Return true if word_1 and word_2 are anagrams.
pub fn is_anagram(word_1: &str, word_2: &str) -> bool {
    let word_1_lower = word_1.to_lowercase();
    let word_2_lower = word_2.to_lowercase();

    let mut word_1_letters: Vec<char> = word_1_lower.chars().collect();
    word_1_letters.sort_unstable();

    let mut word_2_letters: Vec<char> = word_2_lower.chars().collect();
    word_2_letters.sort_unstable();

    (word_1_lower != word_2_lower) && (word_1_letters == word_2_letters)
}

/// Return an HashSet of anagrams among a dictionary array for a specific word.
pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter(|candidate| is_anagram(word, candidate))
        .copied().collect()
}
