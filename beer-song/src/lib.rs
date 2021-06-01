use std::iter::FromIterator;

pub fn verse(n: u32) -> String {
    return match n {
        0 => {
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_owned()
        }
        1 => {
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_owned()
        }
        2 => {
            format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n", n, n - 1)
        }
        _ => {
            format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n - 1)
        }
    };
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: Vec<String> = vec![];
    for n in (end..start + 1).rev() {
        verses.push(verse(n))
    }

    verses.join("\n")
}
