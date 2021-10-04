#![allow(dead_code)]
use midi_format::midi::Midi;

use crate::midi_format::{header::Header, tool};
mod midi_format;

fn main() {
    make_file();
}

fn make_file() {
    let name = "midi-1.mid".to_string();
    let mut midi_file = Midi::new(1, 1, 96);

    // do -> 60
    midi_file.note_on(0, 0, 60, 96);
    midi_file.note_off(0, 96, 60, 0);

    tool::save_bytes(name.to_string(), midi_file.to_bytes()).expect("Err: file cannot be created");

    println!("file created: {}", name);
}

fn fun_with_rust() {
    let quanty = 1_000_000;
    let bytes = tool::to_variable_length_quantity(&quanty);
    println!("{:?}", quanty);
    for byte in &bytes {
        println!("{:#04x}", byte);
    }
    println!("{:#04x?}", bytes);
    let header = Header::new(1000, 1, 96);
    println!("{:#?}", header);
    let a = header.to_byte();
    println!("{:#?}", a);
}
