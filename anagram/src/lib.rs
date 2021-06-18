use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    // possible_anagrams.contains(&word);
    let mut output = HashSet::new();

    for i in 0..possible_anagrams.len() {
        if possible_anagrams[i].len() == word.len() && possible_anagrams[i] != word {
            output.insert(word);
        }
    }

    output
}
