mod util;
mod puzzle;
mod day01;
mod day02;

use day01::Day01;
use day02::Day02;
use puzzle::Puzzle;

fn main() {
    println!("Advent of Code 2017\n");
    Day01::new().solve();
    Day02::new().solve();
}
