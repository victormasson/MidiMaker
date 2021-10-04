#[derive(Debug)]
pub enum Note {
    C1,
    Csharp1,
    D1,
    Dsharp1,
    E1,
    F1,
    Fsharp1,
    G1,
    Gsharp1,
    A1,
    Asharp1,
    B1,

    C2,
    Csharp2,
    D2,
    Dsharp2,
    E2,
    F2,
    Fsharp2,
    G2,
    Gsharp2,
    A2,
    Asharp2,
    B2,

    C3,
    Csharp3,
    D3,
    Dsharp3,
    E3,
    F3,
    Fsharp3,
    G3,
    Gsharp3,
    A3,
    Asharp3,
    B3,

    C4,
    Csharp4,
    D4,
    Dsharp4,
    E4,
    F4,
    Fsharp4,
    G4,
    Gsharp4,
    A4,
    Asharp4,
    B4,

    C5,
    Csharp5,
    D5,
    Dsharp5,
    E5,
    F5,
    Fsharp5,
    G5,
    Gsharp5,
    A5,
    Asharp5,
    B5,

    C6,
    Csharp6,
    D6,
    Dsharp6,
    E6,
    F6,
    Fsharp6,
    G6,
    Gsharp6,
    A6,
    Asharp6,
    B6,
}

impl Note {
    pub fn get_value(self) -> u8 {
        match self {
            Note::C1 => 24,
            Note::Csharp1 => 25,
            Note::D1 => 26,
            Note::Dsharp1 => 27,
            Note::E1 => 28,
            Note::F1 => 29,
            Note::Fsharp1 => 30,
            Note::G1 => 31,
            Note::Gsharp1 => 32,
            Note::A1 => 33,
            Note::Asharp1 => 34,
            Note::B1 => 35,

            Note::C2 => 36,
            Note::Csharp2 => 37,
            Note::D2 => 38,
            Note::Dsharp2 => 39,
            Note::E2 => 40,
            Note::F2 => 41,
            Note::Fsharp2 => 42,
            Note::G2 => 43,
            Note::Gsharp2 => 44,
            Note::A2 => 45,
            Note::Asharp2 => 46,
            Note::B2 => 47,

            Note::C3 => 48,
            Note::Csharp3 => 49,
            Note::D3 => 50,
            Note::Dsharp3 => 51,
            Note::E3 => 52,
            Note::F3 => 53,
            Note::Fsharp3 => 54,
            Note::G3 => 55,
            Note::Gsharp3 => 56,
            Note::A3 => 57,
            Note::Asharp3 => 58,
            Note::B3 => 59,

            Note::C4 => 60,
            Note::Csharp4 => 61,
            Note::D4 => 62,
            Note::Dsharp4 => 63,
            Note::E4 => 64,
            Note::F4 => 65,
            Note::Fsharp4 => 66,
            Note::G4 => 67,
            Note::Gsharp4 => 68,
            Note::A4 => 69,
            Note::Asharp4 => 70,
            Note::B4 => 71,

            Note::C5 => 72,
            Note::Csharp5 => 73,
            Note::D5 => 74,
            Note::Dsharp5 => 75,
            Note::E5 => 76,
            Note::F5 => 77,
            Note::Fsharp5 => 78,
            Note::G5 => 79,
            Note::Gsharp5 => 80,
            Note::A5 => 81,
            Note::Asharp5 => 82,
            Note::B5 => 83,

            Note::C6 => 84,
            Note::Csharp6 => 85,
            Note::D6 => 86,
            Note::Dsharp6 => 87,
            Note::E6 => 88,
            Note::F6 => 89,
            Note::Fsharp6 => 90,
            Note::G6 => 91,
            Note::Gsharp6 => 92,
            Note::A6 => 93,
            Note::Asharp6 => 94,
            Note::B6 => 95,
        }
    }
}
