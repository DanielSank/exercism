use std::convert::From;

#[derive(Clone, Copy)]
enum Note {
    A,
    As,
    Bb,
    B,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
}


impl Note {
    fn number(&self) -> u8 {
        use Note::*;
        match self {
            A => 0,
            As | Note::Bb => 1,
            B => 2,
            C => 3,
            Cs | Note::Db => 4,
            D => 5,
            Ds | Note::Eb => 6,
            E => 7,
            F => 8,
            Fs | Note::Gb => 9,
            G => 10,
            Gs | Note::Ab => 11,
        }
    }

    fn from_number(value: u8, sharps: bool) -> Result<Note, MyError> {
        use Note::*;
        let note = match value % 12 {
            0 => A,
            1 => match sharps {
                true => As,
                false => Bb,
            },
            2 => B,
            3 => C,
            4 => match sharps {
                true => Cs,
                false => Db,
            },
            5 => D,
            6 => match sharps {
                true => Ds,
                false => Eb,
            },
            7 => E,
            8 => F,
            9 => match sharps {
                true => Fs,
                false => Gb,
            },
            10 => G,
            11 => match sharps {
                true => Gs,
                false => Ab,
            },
            _ => return Err(MyError::NoteFromNumber)
        };
        Ok(note)
    }
}


impl TryFrom<&str> for Note {
    type Error = MyError;
    fn try_from(item: &str) -> Result<Note, MyError> {
        use Note::*;
        let note = match item {
            "A" | "a" => A,
            "A#" | "a#" => As,
            "Bb" | "bb" => Bb,
            "B" | "b" => B,
            "C" | "c" => C,
            "C#" | "c#" => Cs,
            "Db" | "db" => Db,
            "D" | "d" => D,
            "D#" | "d#" => Ds,
            "Eb" | "eb" => Eb,
            "E" | "e" => E,
            "F" | "f" => F,
            "F#" | "f#" => Fs,
            "Gb" | "gb" => Gb,
            "G" | "g" => G,
            "G#" | "g#" => Gs,
            "Ab" | "ab" => Ab,
            _ => return Err(MyError::NoteFromStr)
        };
        Ok(note)
    }
}


impl From<Note> for String{
    fn from(item: Note) -> String {
        use Note::*;
        let s = match item {
            A => "A",
            As => "A#",
            Bb => "Bb",
            B => "B",
            C => "C",
            Cs => "C#",
            Db => "Db",
            D => "D",
            Ds => "D#",
            Eb => "Eb",
            E => "E",
            F => "F",
            Fs => "F#",
            Gb => "Gb",
            G => "G",
            Gs => "G#",
            Ab => "Ab",
        };
        String::from(s)
    }
}


impl From<&Note> for String{
    fn from(item: &Note) -> String {
        String::from(*item)
    }
}

#[repr(u8)]
enum Interval {
    Major = 2,
    Minor = 1,
    Augmented = 3,
}

impl TryFrom<char> for Interval {
    type Error = MyError;
    fn try_from(interval: char) -> Result<Self, MyError> {
        use Interval::*;
        match interval {
            'M' => Ok(Major),
            'm' => Ok(Minor),
            'A' => Ok(Augmented),
            _ => Err(MyError::IntervalFromChar),
        }
    }
}


#[derive(Debug)]
pub enum MyError {
    IntervalFromChar,
    NoteFromNumber,
    NoteFromStr,
    TonicNotKnown,
}

pub struct Scale {
    notes: Vec<Note>
}


fn sharps(tonic: &str) -> Result<bool, MyError> {
    let result = match tonic {
        "C" | "G" | "D" | "A" | "E" | "B" | "F#" | "a" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => true,
        "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => false,
        _ => return Err(MyError::TonicNotKnown),
    };
    Ok(result)
}


impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, MyError> {
        let sharps = sharps(tonic)?;
        let tonic = Note::try_from(tonic)?;
        let mut notes = Vec::new();
        notes.push(tonic);
        let mut step = 0;
        for interval in intervals.chars() {
            step = step + Interval::try_from(interval)? as u8;
            notes.push(Note::from_number(tonic.number() + step, sharps)?);
        }
        Ok(Scale{notes})
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, MyError> {
        let sharps = sharps(tonic)?;
        let tonic = Note::try_from(tonic)?;
        let mut notes = Vec::new();
        for step in 0..=12 {
            notes.push(Note::from_number(tonic.number() + step, sharps)?);
        }
        Ok(Scale{notes})
    }

    pub fn enumerate(&self) -> Vec<String> {
        let mut notes = Vec::new();
        for note in &self.notes {
            notes.push(String::from(note))
        }
        notes
    }
}
