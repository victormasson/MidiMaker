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

    pub fn end_track() -> TrackEvent {
        TrackEvent {
            delta_time: 0,
            data: vec![0xFF, 0x2F, 0x00], // code to finish track
        }
    }
}
