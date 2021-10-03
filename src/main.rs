#[warn(non_snake_case)]
mod midi_format;
use crate::midi_format::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let quanty = 1_000_000;
    let bytes = to_variable_length_quantity(&quanty);

    println!("{:?}", quanty);

    for byte in &bytes {
        println!("{:#04x}", byte);
    }
    println!("{:#04x?}", bytes);

    let header = header::Header::new(1000, 1, 96);
    println!("{:#?}", header);

    let a = header.to_byte();
    println!("{:#?}", a);

    save_bytes("midi-1.mid".to_string(), a).expect("Err: file cannot be created");
}

fn to_variable_length_quantity(value: &u32) -> Vec<u8> {
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

    bytes
}

fn save_bytes(file_name: String, bytes: Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(&file_name)?;
    file.write_all(bytes.as_slice())?;
    Ok(())
}
