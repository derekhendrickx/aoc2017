use std::i32;
use util::open_file_to_vector;
use puzzle::Puzzle;

pub struct Day02 {
    input: Vec<Vec<i32>>
}

impl Day02 {
    pub fn new() -> Day02 {
        Day02 { input: open_file_to_vector("src/day02/input.txt") }
    }
}

impl Puzzle for Day02 {
    fn solve(&self) {
        println!("Day 02");
        let sum = difference_between_max_min(&self.input);
        println!("Sum = {}", sum);
    }
}

fn difference_between_max_min(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;

    for row in input {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for x in row {
            if *x > max {
                max = *x;
            }
            
            if *x < min {
                min = *x;
            }
        }

        sum += max - min;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::difference_between_max_min;

    #[test]
    fn test_difference_between_max_min_sample_input() {
        let input = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8]
        ];
        let sum = difference_between_max_min(&input);
        assert_eq!(18, sum);
    }
}
