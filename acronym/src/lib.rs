use regex::Regex;

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
        let capitalized_sub_words = split_by_case_without_apostrophe(word);

        for word in capitalized_sub_words {
            acronym_part.push(get_capitalized_first_letter(word.as_str()));
        }
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

fn split_by_case_without_apostrophe(s: &str) -> Vec<String> {
    let s = s.replace("'", "");
    let re = Regex::new(r"([A-Z][a-z]+)|([a-z]+)|([A-Z]+)").unwrap();
    let mut result = vec![];
    for cap in re.captures_iter(&s) {
        result.push(cap[0].to_string());
    }

    result
}