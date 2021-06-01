// use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let rev_word_chars = input.chars().rev().collect();
    // let rev_word_unicode_segmentation: String = input.graphemes(true).collect::<String>();

    rev_word_chars
}
