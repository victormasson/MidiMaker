use std::{fs::File, io::Write};

pub fn to_variable_length_quantity(value: &u32) -> Vec<u8> {
    let mut temp = *value;
    let mut bytes = vec![];

    while temp > 0 {
        let mut byte = (temp & 0b0111_1111) as u8;
        if bytes.len() > 0 {
            byte += 0b1000_0000;
        }

        bytes.push(byte);
        temp = temp >> 7;
    }

    if bytes.len() == 0 {
        bytes.push(0);
    }

    bytes
}

pub fn save_bytes(file_name: String, bytes: Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(&file_name)?;
    file.write_all(bytes.as_slice())?;
    Ok(())
}
