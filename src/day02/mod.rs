use std::i32;
use util::open_file_to_vector;
use puzzle::Puzzle;

pub struct Day02 {
    input: Vec<Vec<i32>>,
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 { input: open_file_to_vector("src/day02/input.txt") }
    }
}

impl Puzzle for Day02 {
    fn solve(&self) {
        let sum_part1 = difference_between_max_min(&self.input);
        let sum_part2 = evenly_divide(&self.input);

        println!("Day 02");
        println!("Sum Part 1 = {}", sum_part1);
        println!("Sum Part 2 = {}", sum_part2);
    }
}

fn difference_between_max_min(input: &[Vec<i32>]) -> i32 {
    let mut sum: i32 = 0;

    input
        .iter()
        .for_each(|row| {
            let min = row.iter().min().unwrap();
            let max = row.iter().max().unwrap();
            sum += max - min;
        });

    sum
}

fn evenly_divide(input: &[Vec<i32>]) -> i32 {
    let mut sum: i32 = 0;

    for row in input {
        for x in row {
            for y in row {
                if *x != *y && *x % *y == 0 {
                    sum += x / y;
                    break;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::difference_between_max_min;
    use super::evenly_divide;

    #[test]
    fn test_difference_between_max_min_sample_input() {
        let input = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        let sum = difference_between_max_min(&input);
        assert_eq!(18, sum);
    }

    #[test]
    fn test_evenly_divide_sample_input() {
        let input = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        let sum = evenly_divide(&input);
        assert_eq!(9, sum);
    }
}
