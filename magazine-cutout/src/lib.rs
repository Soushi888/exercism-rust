use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map = HashMap::new();
    let mut note_map = HashMap::new();

    for word in magazine {
        let count = magazine_map.entry(word).or_insert(0);
        *count += 1;
    }

    for word in note {
        let count = note_map.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in note_map {
        if magazine_map.get(word).unwrap_or(&0) < &count {
            return false;
        }
    }

    true
}
