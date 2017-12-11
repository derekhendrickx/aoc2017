use std::io::prelude::*;
use std::fs::File;

fn read_file(file: &str) -> String {
    let mut input_file = File::open(file).expect("Unable to open file");
    let mut buffer = String::new();

    input_file.read_to_string(&mut buffer).expect(
        "Unable to read data",
    );

    buffer
}

pub fn open_file(file: &str) -> Vec<i32> {
    let buffer = read_file(file);
    let mut input = Vec::new();

    for digit in buffer.chars() {
        if let Some(value) = digit.to_digit(10) {
            input.push(value as i32);
        }
    }

    input
}

pub fn open_file_to_vector(file: &str) -> Vec<Vec<i32>> {
    let buffer = read_file(file);
    let mut input: Vec<Vec<i32>> = Vec::new();

    for values in buffer.lines() {
        let row_as_string: Vec<&str> = values.split('\t').collect();
        let mut row: Vec<i32> = Vec::new();

        for value in row_as_string {
            row.push(value.parse().unwrap());
        }

        input.push(row.clone())
    }

    input
}
