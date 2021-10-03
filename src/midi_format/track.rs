use crate::track_event::TrackEvent;

pub struct Track {
    chunk_type: String,
    events: Vec<TrackEvent>,
    channel: u8,
}

impl Track {
    pub fn new(channel: u8) -> Track {
        let mut track = Track {
            chunk_type: "MTrk".to_string(),
            channel,
            events: vec![],
        };

        track.add_event(TrackEvent::end_track());
        track
    }

    fn add_event(&mut self, event: TrackEvent) {
        if self.events.len() == 0 {
            self.events.push(event);
        } else {
            self.events.insert(self.events.len() - 1, event);
        }
    }
}
