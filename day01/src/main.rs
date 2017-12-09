use std::io::prelude::*;
use std::fs::File;

fn find_sum_match_next_digit(inputs: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let mut previous = inputs[0];
    let last = inputs[inputs.len() - 1];

    if last == previous {
        sum += last;
    }

    for x in &inputs[1..] {
        if *x == previous {
            sum += *x;
        }
        previous = *x;
    }

    sum
}

fn main() {
    let mut input_file = File::open("input/input.txt").expect("Unable to open file");
    let mut buffer = String::new();
    let mut input = Vec::new();

    input_file.read_to_string(&mut buffer).expect(
        "Unable to read data",
    );
    for digit in buffer.chars() {
        match digit.to_digit(10) {
            Some(value) => input.push(value as i32),
            None => (),
        }
    }
    let sum = find_sum_match_next_digit(&input);

    println!("Inputs = {:?}", input);
    println!("Sum = {}", sum);
}
