use std::io::prelude::*;
use std::fs::File;

mod day01;

use day01::{find_sum_match_next_digit, find_sum_match_halfway_round};

fn main() {
    let mut input_file = File::open("src/day01/input.txt").expect("Unable to open file");
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
    let sum_part1 = find_sum_match_next_digit(&input);
    let sum_part2 = find_sum_match_halfway_round(&input);

    println!("Sum Part 1 = {}", sum_part1);
    println!("Sum Part 2 = {}", sum_part2);
}
