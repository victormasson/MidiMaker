use super::track_event::TrackEvent;

#[derive(Debug, Clone)]
pub struct Track {
    chunk_type: String,
    events: Vec<TrackEvent>,
    channel: u8,
}

impl Track {
    pub fn new(channel: u8) -> Track {
        Track {
            chunk_type: "MTrk".to_string(),
            channel,
            events: vec![TrackEvent::end_track()],
        }
    }

    pub fn note_on(&mut self, delta_time: u32, note: u8, velocity: u8) {
        self.add_event(TrackEvent::note_on(
            delta_time,
            self.channel,
            note,
            velocity,
        ));
    }

    pub fn note_off(&mut self, delta_time: u32, note: u8) {
        self.add_event(TrackEvent::note_off(delta_time, self.channel, note, 0));
    }

    fn add_event(&mut self, event: TrackEvent) {
        match self.events.len() {
            0 => self.events.push(event),
            _ => self.events.insert(self.events.len() - 1, event),
        };
    }

    pub fn len(&self) -> u32 {
        self.events.iter().map(|event| event.len()).sum()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(self.chunk_type.as_bytes());
        bytes.extend_from_slice(&self.len().to_be_bytes());

        self.events
            .iter()
            .for_each(|event| bytes.append(&mut event.to_bytes()));

        bytes
    }
}

#[cfg(test)]
mod test {
    use crate::midi::{track::Track, track_event::TrackEvent};

    #[test]
    fn track_test() {
        let mut t = Track::new(127);
        let e = TrackEvent::note_on(12, 12, 12, 12);
        let eaze = e.clone();

        t.add_event(e);
        assert_eq!(2, t.events.len());

        let ef = format!("{:#?}", eaze);
        let other = format!("{:#?}", t.events.first().unwrap());
        assert_eq!(ef, other);
    }

    #[test]
    fn len_test() {
        let mut t = Track::new(127);
        let e = TrackEvent::note_on(12, 12, 12, 12);
        t.add_event(e);

        let res = t.len();

        let mut length = 0;
        t.events.iter().for_each(|event| length += event.len());

        println!("{} {}", res, length);
    }
}
