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

fn get_notes(flats: bool) -> [&'static str; 12] {
    if flats {
        ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]
    } else {
        ["F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E"]
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        todo!()
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let tonic = tonic.to_uppercase();
        let tonic_index = get_notes(true).iter().position(|&x| x == tonic).ok_or(Error)?;

        let mut scale: Vec<String> = get_notes(true)[tonic_index..].iter().map(|&x| x.to_string()).collect();
        scale.extend_from_slice(get_notes(true)[..tonic_index].iter().map(|&x| x.to_string()).collect::<Vec<String>>().as_slice());
        scale.push(tonic);


        Ok(Scale(scale))
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}
