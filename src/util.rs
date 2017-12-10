use std::io::prelude::*;
use std::fs::File;

pub fn open_file(file: &str) -> Vec<i32> {
    let mut input_file = File::open(file).expect("Unable to open file");
    let mut buffer = String::new();
    let mut input = Vec::new();

    input_file.read_to_string(&mut buffer).expect(
        "Unable to read data",
    );

    for digit in buffer.chars() {
        if let Some(value) = digit.to_digit(10) {
            input.push(value as i32);
        }
    }

    input
}
