use crate::midi_format::tool;

#[derive(Debug, Clone)]
pub struct TrackEvent {
    delta_time: u32,
    data: Vec<u8>,
}

impl TrackEvent {
    pub fn note_on(delta_time: u32, channel: u8, note: u8, velocity: u8) -> TrackEvent {
        let start = 0x90 + (channel & 0xF);

        TrackEvent {
            delta_time,
            data: vec![start, note, velocity],
        }
    }

    pub fn note_off(delta_time: u32, channel: u8, note: u8, velocity: u8) -> TrackEvent {
        let start = 0x80 + (channel & 0xF);

        TrackEvent {
            delta_time,
            data: vec![start, note, velocity],
        }
    }

    pub fn end_track() -> TrackEvent {
        TrackEvent {
            delta_time: 0,
            data: vec![0xFF, 0x2F, 0x00], // code to finish track
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&tool::to_variable_length_quantity(&self.delta_time));
        bytes.extend_from_slice(self.data.as_slice());

        bytes
    }

    pub fn len(&self) -> u32 {
        self.to_bytes().len() as u32
    }
}
