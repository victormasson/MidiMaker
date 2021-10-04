#[derive(Debug)]
pub struct Header {
    chunk_type: String,
    length: u32,
    format: u16,
    nb_tracks: u16,
    division: u16,
}

impl Header {
    pub fn new(format: u16, nb_tracks: u16, division: u16) -> Header {
        Header {
            chunk_type: "MThd".to_string(),
            length: 6,
            format,
            nb_tracks,
            division,
        }
    }

    pub fn to_byte(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend_from_slice(&self.chunk_type.as_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.format.to_be_bytes());
        bytes.extend_from_slice(&self.nb_tracks.to_be_bytes());
        bytes.extend_from_slice(&self.division.to_be_bytes());

        bytes
    }
}
