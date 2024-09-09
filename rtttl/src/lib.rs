#![no_std]

pub mod note;

#[derive(Debug)]
pub struct Song<'a> {
    title: &'a str,
    duration: u32,
    octave: u32,
    beats_per_minute: u32,
    notes: core::iter::Peekable<core::str::Split<'a, &'a str>>,
    time: u32,
}

impl<'a> Song<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut split = text.splitn(3, ':');
        let title = split.next().unwrap().trim();
        let mut settings = split.next().unwrap().trim().splitn(3, ',').map(|setting| {
            let s = setting.split_once("=").unwrap();
            Some((s.0.trim(), s.1.trim()))
        });
        let notes = split.next().unwrap().trim().split(",").peekable();
        let mut duration = 4;
        let mut octave = 5;
        let mut beats_per_minute = 108;

        for setting in settings {
            match setting {
                Some(("o", o)) | Some(("O", o)) => octave = o.parse().unwrap(),
                Some(("d", d)) | Some(("D", d)) => duration = d.parse().unwrap(),
                Some(("b", b)) | Some(("B", b)) => beats_per_minute = b.parse().unwrap(),
                Some((k, v)) => panic!("panic {} {}", k, v),
                None => panic!("panic"),
            }
        }

        Self {
            title,
            duration,
            octave,
            beats_per_minute,
            notes,
            time: 0,
        }
    }

    pub fn note_at(&mut self, time_ms: u32) -> Option<note::Note> {
        let note = note::Note::new(self.notes.peek().unwrap(), self.octave, self.duration);
        self.time += note.duration(self.beats_per_minute);
        if self.time > time_ms {
            return Some(note);
        }

        None
    }
}

// TODO: expand tests
#[cfg(test)]
mod test {
    use super::*;

    const HAUNTED_HOUSE: &'static str = "HauntHouse: d=4,o=5,b=108: 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4, 1p, 2a4, 2e, 2d#, 2b4, 2a4, 2c, 2d, 2a#4, 2e., e, 1f4, 1a4, 1d#, 2e., d, 2c., b4, 1a4";
    const COUNTDOWN: &'static str = "countdown:d=4, o=5, b=125:p, 8p, 16b, 16a, b, e, p, 8p, 16c6, 16b, 8c6, 8b, a, p, 8p, 16c6, 16b, c6, e, p, 8p, 16a, 16g, 8a, 8g, 8f#, 8a, g., 16f#, 16g, a., 16g, 16a, 8b, 8a, 8g, 8f#, e, c6, 2b., 16b, 16c6, 16b, 16a, 1b";
    const MISSION: &'static str = "Mission:d=4, o=6, b=100:32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d#, 32d, 32d, 32d#, 32e, 32f, 32f#, 32g, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16g, 8p, 16g, 8p, 16a#, 16p, 16c, 16p, 16g, 8p, 16g, 8p, 16f, 16p, 16f#, 16p, 16a#, 16g, 2d, 32p, 16a#, 16g, 2c#, 32p, 16a#, 16g, 2c, 16p, 16a#5, 16c";

    #[test]
    fn test_countdown() {
        let mut song = Song::new(COUNTDOWN);

        assert_eq!(song.title, "countdown");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 5);
        assert_eq!(song.beats_per_minute, 125);
    }

    #[test]
    fn test_song() {
        let mut song = Song::new(HAUNTED_HOUSE);

        assert_eq!(song.title, "HauntHouse");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 5);
        assert_eq!(song.beats_per_minute, 108);
    }

    #[test]
    fn test_mission() {
        let mut song = Song::new(MISSION);

        assert_eq!(song.title, "Mission");
        assert_eq!(song.duration, 4);
        assert_eq!(song.octave, 6);
        assert_eq!(song.beats_per_minute, 100);
    }
}
