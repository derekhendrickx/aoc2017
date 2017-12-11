use util::open_file;
use puzzle::Puzzle;

pub struct Day01 {
    input: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Day01 {
        Day01 { input: open_file("src/day01/input.txt") }
    }
}

impl Puzzle for Day01 {
    fn solve(&self) {
        let sum_part1 = find_sum_match_next_digit(&self.input);
        let sum_part2 = find_sum_match_halfway_round(&self.input);

        println!("Day 01");
        println!("Sum Part 1 = {}", sum_part1);
        println!("Sum Part 2 = {}", sum_part2);
    }
}

fn find_sum_match_next_digit(inputs: &[i32]) -> i32 {
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

fn find_sum_match_halfway_round(input: &[i32]) -> i32 {
    let half = ((input.len() as f32) * 0.5) as usize;
    let first_half = &input[0..half];
    let second_half = &input[half..input.len()];
    let mut sum: i32 = 0;

    for (index, x) in first_half.iter().enumerate() {
        if *x == second_half[index] {
            sum += *x;
        }
    }

    for (index, x) in second_half.iter().enumerate() {
        if *x == first_half[index] {
            sum += *x;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::find_sum_match_next_digit;
    use super::find_sum_match_halfway_round;

    #[test]
    fn test_find_sum_match_next_digit_input_sample_1() {
        let input = [1, 1, 2, 2];
        let sum = find_sum_match_next_digit(&input);
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_find_sum_match_next_digit_input_sample_2() {
        let input = [1, 1, 1, 1];
        let sum = find_sum_match_next_digit(&input);
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_find_sum_match_next_digit_input_sample_3() {
        let input = [1, 2, 3, 4];
        let sum = find_sum_match_next_digit(&input);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_find_sum_match_next_digit_input_sample_4() {
        let input = [9, 1, 2, 1, 2, 1, 2, 9];
        let sum = find_sum_match_next_digit(&input);
        assert_eq!(sum, 9);
    }

    #[test]
    fn test_find_sum_match_halfway_round_input_sample_1() {
        let input = [1, 2, 1, 2];
        let sum = find_sum_match_halfway_round(&input);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_find_sum_match_halfway_round_input_sample_2() {
        let input = [1, 2, 3, 4];
        let sum = find_sum_match_halfway_round(&input);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_find_sum_match_halfway_round_input_sample_3() {
        let input = [1, 2, 3, 4, 2, 5];
        let sum = find_sum_match_halfway_round(&input);
        assert_eq!(sum, 4);
    }

    #[test]
    fn test_find_sum_match_halfway_round_input_sample_4() {
        let input = [1, 2, 3, 1, 2, 3];
        let sum = find_sum_match_halfway_round(&input);
        assert_eq!(sum, 12);
    }

    #[test]
    fn test_find_sum_match_halfway_round_input_sample_5() {
        let input = [1, 2, 1, 3, 1, 4, 1, 5];
        let sum = find_sum_match_halfway_round(&input);
        assert_eq!(sum, 4);
    }
}
