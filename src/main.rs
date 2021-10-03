#![allow(non_snake_case)]

fn main() {
    let quanty = 1_000_000;
    let bytes = toVariableLengthQuantity(&quanty);

    println!("{:?}", quanty);

    for byte in &bytes {
        println!("{:#04x}", byte);
    }
    println!("{:#04x?}", bytes);
}

fn toVariableLengthQuantity(value: &u32) -> Vec<u8> {
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
