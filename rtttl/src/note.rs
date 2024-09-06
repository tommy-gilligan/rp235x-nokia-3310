#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NoteName {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    Pause
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNoteNameError;

use core::str::FromStr;

impl FromStr for NoteName {
    type Err = ParseNoteNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" | "a" => Ok(NoteName::A),
            "A#" | "a#" => Ok(NoteName::ASharp),
            "B" | "b" => Ok(NoteName::B),
            "C" | "c" => Ok(NoteName::C),
            "C#" | "c#" => Ok(NoteName::CSharp),
            "D" | "d" => Ok(NoteName::D),
            "D#" | "d#" => Ok(NoteName::DSharp),
            "E" | "e" => Ok(NoteName::E),
            "F" | "f" => Ok(NoteName::F),
            "F#" | "f#" => Ok(NoteName::FSharp),
            "G" | "g" => Ok(NoteName::G),
            "G#" | "g#" => Ok(NoteName::GSharp),
            "P" | "p" => Ok(NoteName::Pause),
            _ => Err(ParseNoteNameError)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Note(u32, NoteName, u32, bool);

impl Note {
    pub fn new<'a>(text: &'a str, default_octave: u32, default_duration: u32) -> Self {
        let mut n = text.trim();
        let mut find_name = n.match_indices(|c: char| !c.is_digit(10));
        let (name_index, name) = find_name.next().unwrap();

        Note(
            n[name_index + name.len()..].parse().unwrap_or(default_duration),
            name.parse().unwrap(),
            n[..name_index].parse().unwrap_or(default_octave),
            n.ends_with(".")
        )
    }

    pub fn duration(&self, beats_per_minute: u32) -> u32 {
        if self.3 {
            2 * 240 * 1000 / (3 * beats_per_minute * self.2)
        } else {
            240 * 1000 / (beats_per_minute * self.2)
        }
    }

    // TODO: DRY
    pub fn frequency(&self) -> Option<Result<u32, ()>> {
        match (self.0, self.1) {
            (_, NoteName::Pause) => None,
            (3, NoteName::C) => Some(Ok(130)),
            (3, NoteName::CSharp) => Some(Ok(138)),
            (3, NoteName::D) => Some(Ok(146)),
            (3, NoteName::DSharp) => Some(Ok(155)),
            (3, NoteName::E) => Some(Ok(164)),
            (3, NoteName::F) => Some(Ok(174)),
            (3, NoteName::FSharp) => Some(Ok(184)),
            (3, NoteName::G) => Some(Ok(195)),
            (3, NoteName::GSharp) => Some(Ok(207)),
            (3, NoteName::A) => Some(Ok(220)),
            (3, NoteName::ASharp) => Some(Ok(233)),
            (3, NoteName::B) => Some(Ok(246)),
            (4, NoteName::C) => Some(Ok(261)),
            (4, NoteName::CSharp) => Some(Ok(277)),
            (4, NoteName::D) => Some(Ok(293)),
            (4, NoteName::DSharp) => Some(Ok(311)),
            (4, NoteName::E) => Some(Ok(329)),
            (4, NoteName::F) => Some(Ok(349)),
            (4, NoteName::FSharp) => Some(Ok(369)),
            (4, NoteName::G) => Some(Ok(391)),
            (4, NoteName::GSharp) => Some(Ok(415)),
            (4, NoteName::A) => Some(Ok(440)),
            (4, NoteName::ASharp) => Some(Ok(466)),
            (4, NoteName::B) => Some(Ok(493)),
            (5, NoteName::C) => Some(Ok(523)),
            (5, NoteName::CSharp) => Some(Ok(554)),
            (5, NoteName::D) => Some(Ok(587)),
            (5, NoteName::DSharp) => Some(Ok(622)),
            (5, NoteName::E) => Some(Ok(659)),
            (5, NoteName::F) => Some(Ok(698)),
            (5, NoteName::FSharp) => Some(Ok(739)),
            (5, NoteName::G) => Some(Ok(783)),
            (5, NoteName::GSharp) => Some(Ok(830)),
            (5, NoteName::A) => Some(Ok(880)),
            (5, NoteName::ASharp) => Some(Ok(932)),
            (5, NoteName::B) => Some(Ok(987)),
            (6, NoteName::C) => Some(Ok(1046)),
            (6, NoteName::CSharp) => Some(Ok(1108)),
            (6, NoteName::D) => Some(Ok(1174)),
            (6, NoteName::DSharp) => Some(Ok(1244)),
            (6, NoteName::E) => Some(Ok(1318)),
            (6, NoteName::F) => Some(Ok(1396)),
            (6, NoteName::FSharp) => Some(Ok(1479)),
            (6, NoteName::G) => Some(Ok(1567)),
            (6, NoteName::GSharp) => Some(Ok(1661)),
            (6, NoteName::A) => Some(Ok(1760)),
            (6, NoteName::ASharp) => Some(Ok(1864)),
            (6, NoteName::B) => Some(Ok(1975)),
            (o, n) => { Some(Err(())) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_note() {
        assert_eq!(Note::new("2a4", 5, 4), Note(2, NoteName::A, 4, false));
    }
}

