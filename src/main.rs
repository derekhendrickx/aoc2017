mod util;
mod puzzle;
mod day01;

use day01::Day01;
use puzzle::Puzzle;

fn main() {
    let day01 = Day01::new();
    day01.solve();
}
