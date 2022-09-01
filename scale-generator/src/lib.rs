// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale(Vec<String>);

fn get_notes(sharps: bool, tonic: &str) -> Vec<String> {
    let notes;
    match sharps {
        true => notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"],
        false => notes = ["F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E"],
    }

    let tonic = tonic.to_uppercase();
    let tonic_index = notes.iter().position(|&x| x == tonic).unwrap();

    let mut scale: Vec<String> = notes[tonic_index..].iter().map(|&x| x.to_string()).collect();
    scale.extend_from_slice(notes[..tonic_index].iter().map(|&x| x.to_string()).collect::<Vec<String>>().as_slice());
    scale.push(tonic.to_string());

    scale
}


impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        todo!()
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let scale = match tonic {
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "Cb" => Scale(get_notes(false, tonic)),
            _ => Scale(get_notes(true, tonic)),
        };

        Ok(scale)
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
