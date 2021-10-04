#![allow(dead_code)]
use midi::midi::Midi;

use crate::midi::{
    header::Header,
    music::{dynamics::Dynamics, note::Note},
    tool,
};
mod midi;

fn main() {
    make_file();
}

fn make_file() {
    let name = "midi-1.mid".to_string();
    let mut midi_file = Midi::new(1, 1, 96);

    // do -> 60
    midi_file.note_on(0, 0, Note::C3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::C3);

    midi_file.note_on(0, 0, Note::C3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::C3);

    midi_file.note_on(0, 0, Note::C3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::C3);

    midi_file.note_on(0, 0, Note::D3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::D3);

    midi_file.note_on(0, 0, Note::E3, Dynamics::mf);
    midi_file.note_off(0, 48 * 2, Note::E3);

    midi_file.note_on(0, 0, Note::D3, Dynamics::mf);
    midi_file.note_off(0, 48 * 2, Note::D3);

    midi_file.note_on(0, 0, Note::C3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::C3);

    midi_file.note_on(0, 0, Note::E3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::E3);

    midi_file.note_on(0, 0, Note::D3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::D3);

    midi_file.note_on(0, 0, Note::D3, Dynamics::mf);
    midi_file.note_off(0, 48, Note::D3);

    midi_file.note_on(0, 0, Note::C3, Dynamics::mf);
    let a = 96 + 10;
    midi_file.note_off(0, a, Note::C3);

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
