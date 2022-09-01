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

#[derive(Clone, Debug)]
pub struct Scale(Vec<String>);

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let notes = Scale::chromatic(tonic)?.0;
        let mut scale: Vec<String> = vec![notes[0].clone()];

        let mut position = 0;
        for interval in intervals.chars() {
            if interval == 'm' {
                position += 1;
            } else if interval == 'M' {
                position += 2;
            }
            scale.push(notes[position].clone());
        }

        Ok(Scale(scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let uppercase_tonic = tonic.chars().next().unwrap().to_uppercase().collect::<String>() + &tonic[1..];
        let tonic = uppercase_tonic.as_str();

        let scale = match tonic {
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "Cb" => Scale(get_chromatic_octave(false, tonic)),
            _ => Scale(get_chromatic_octave(true, tonic)),
        };

        Ok(scale)
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
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