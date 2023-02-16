pub fn abbreviate(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();

    let mut acronym = String::new();
    for word in words {
        let acronym_part = get_acronym_part_from_word(word);
        let chars: Vec<char> = acronym_part.chars().collect();
        for char in chars {
            acronym.push(char)
        }
    }

    acronym
}

fn get_acronym_part_from_word(word: &str) -> String {
    let sub_words: Vec<&str> = word.split("-").collect();
    let mut acronym_part: Vec<String> = vec![];
    for word in sub_words {
        acronym_part.push(get_capitalized_first_letter(word));
    }

    acronym_part.join("")
}

fn get_capitalized_first_letter(str: &str) -> String {
    let mut chars = str.chars();
    let first_char = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>(),
    };

    first_char
}