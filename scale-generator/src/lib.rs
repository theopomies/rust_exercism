const SHARPS: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLATS: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];
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
    InvalidInterval,
}

pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let set = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => FLATS,
            _ => Err(Error::InvalidTonic)?,
        };

        let pos = set
            .iter()
            .position(|t| t.to_uppercase() == tonic.to_uppercase()[..])
            .unwrap();

        let notes = intervals
            .chars()
            .try_fold(
                (vec![set[pos].to_owned()], pos),
                |(mut notes, pos), char| {
                    let increment = match char {
                        'm' => 1,
                        'M' => 2,
                        'A' => 3,
                        _ => Err(Error::InvalidInterval)?,
                    };
                    let pos = (pos + increment) % set.len();
                    notes.push(set[pos].into());
                    Ok((notes, pos))
                },
            )?
            .0;

        Ok(Self { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
