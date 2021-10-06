#[derive(Debug)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    HundredTwentyEighth,
}

impl NoteDuration {
    pub fn get_value(self, tempo: u8) -> u8 {
        match self {
            NoteDuration::Whole => (tempo * 1),
            NoteDuration::Half => (tempo * 1) / 2,
            NoteDuration::Quarter => (tempo * 1) / 4,
            NoteDuration::Eighth => (tempo * 1) / 8,
            NoteDuration::Sixteenth => (tempo * 1) / 16,
            NoteDuration::ThirtySecond => (tempo * 1) / 32,
            NoteDuration::SixtyFourth => (tempo * 1) / 64,
            NoteDuration::HundredTwentyEighth => (tempo * 1) / 128,
        }
    }
}
