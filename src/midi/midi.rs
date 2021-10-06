use super::{
    header::Header,
    music::{dynamics::Dynamics, note::Note, note_duration::NoteDuration},
    track::Track,
};

#[derive(Debug)]
pub struct Midi {
    header: Header,
    tracks: Vec<Track>,
}

impl Midi {
    pub fn new(format: u16, nb_tracks: u16, division: u16) -> Midi {
        let mut midi = Midi {
            header: Header::new(format, nb_tracks, division),
            tracks: vec![],
        };

        (0..nb_tracks).for_each(|i| {
            midi.tracks.push(Track::new(i as u8));
        });

        midi
    }

    pub fn write_note(&mut self, track: u8, tempo: u8, note: Note, delta_time: NoteDuration) {
        self.note_on(track.into(), 0, note, Dynamics::mf);
        self.note_off(track.into(), delta_time.get_value(tempo) as u32, note);
    }

    pub fn note_on(&mut self, track: u16, delta_time: u32, note: Note, velocity: Dynamics) {
        self.tracks[track as usize].note_on(delta_time, note.get_value(), velocity.get_value())
    }

    pub fn note_off(&mut self, track: u16, delta_time: u32, note: Note) {
        self.tracks[track as usize].note_off(delta_time, note.get_value())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self.header.to_byte());

        self.tracks.iter().for_each(|track| {
            bytes.append(&mut track.to_bytes());
        });

        bytes
    }
}
