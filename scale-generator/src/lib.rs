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
pub enum Error {
    InvalidTonic,
    InvalidIntervals,
}

#[derive(Clone, Debug)]
pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let is_locrian = intervals == "mMMmMMM";
        let is_harmonic_minor = intervals == "MmMMmAm";

        let notes: Vec<String>;
        if is_locrian || is_harmonic_minor {
            notes = get_chromatic_octave(false, tonic);
        } else {
            notes = match tonic {
                    "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => get_chromatic_octave(true, tonic),
                    "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => get_chromatic_octave(false, tonic),
                    _ => return Err(Error::InvalidTonic),
            };
        }

        let mut scale: Vec<String> = vec![notes[0].clone()];

        let mut position = 0;
        for interval in intervals.chars() {
            match interval {
                'm' => position += 1,
                'M' => position += 2,
                'A' => position += 3,
                _ => return Err(Error::InvalidIntervals),
            }

            scale.push(notes[position].clone());
        }

        Ok(Scale { notes: scale })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}

fn get_chromatic_octave(sharps: bool, tonic: &str) -> Vec<String> {
    let notes = match sharps {
        true => ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"],
        false => ["F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E"],
    };

    let uppercase_tonic = tonic.chars().next().unwrap().to_uppercase().collect::<String>() + &tonic[1..];
    let tonic = uppercase_tonic.as_str();
    let tonic_index = notes.iter().position(|&x| x == tonic).unwrap();

    let mut scale: Vec<String> = notes[tonic_index..].iter().map(|&x| x.to_string()).collect();
    scale.extend_from_slice(notes[..tonic_index].iter().map(|&x| x.to_string()).collect::<Vec<String>>().as_slice());
    scale.push(tonic.to_string());

    scale
}