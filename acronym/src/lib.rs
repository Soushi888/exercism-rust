pub fn abbreviate(phrase: &str) -> String {
	let words: Vec<&str> = phrase.split_whitespace().collect();
	println!("{:?}", words);

	let mut acronym = String::new();
	for word in words {}

	acronym
}

fn get_acronym_part_from_word(word: &str) -> &str {
	let sub_words: Vec<&str> = word.split("-").collect();
	let mut acronym_part = String::new();
	for word in sub_words {
		if word.to_uppercase() == word {
			acronym_part.push_str(&word[..1]);
		} else {
			let mut chars = word.chars();
			if chars.into_iter().next().unwrap().to_uppercase() {
				acronym_part.push(chars.next().unwrap().to_uppercase())
			}
		}
	}


	word
}