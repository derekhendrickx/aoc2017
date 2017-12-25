use day03::Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_manhattan_distance(input: i32) -> i32 {
    let n = (input as f32).sqrt().ceil() as usize;
    let mut spiral_memory = vec![0; n * n];
    let n_square = n * n;
    let mut direction = Left;
    let mut x = n - 1;
    let origin_y = n / 2;
    let origin_index: usize = if n % 2 == 0 {
        x = 0;
        direction = Right;
        (origin_y * n) + (origin_y - 1)
    } else {
        (origin_y * n) + origin_y
    };
    let mut y = x;

    spiral_memory[origin_index] = 1;

    for value in (1..n_square).rev() {
        spiral_memory[(y * n) + x] = (value + 1) as i32;

        match direction {
            Up => {
                if y == 0 || spiral_memory[((y - 1) * n) + x] > 0 {
                    direction = Right;
                }
            }
            Down => {
                if y == n - 1 || spiral_memory[((y + 1) * n) + x] > 0 {
                    direction = Left;
                }
            }
            Left => {
                if x == 0 || spiral_memory[(y * n) + (x - 1)] > 0 {
                    direction = Up;
                }
            }
            Right => {
                if x == n - 1 || spiral_memory[(y * n) + (x + 1)] > 0 {
                    direction = Down;
                }
            }
        }

        match direction {
            Up => {
                y -= 1;
            }
            Down => {
                y += 1;
            }
            Left => {
                x -= 1;
            }
            Right => {
                x += 1;
            }
        }
    }

    let origin_position = spiral_memory.iter().position(|&value| value == 1).unwrap();
    let origin = ((origin_position / n) as i32, (origin_position % n) as i32);
    let input_position = spiral_memory
        .iter()
        .position(|&value| value == input)
        .unwrap();
    let destination = ((input_position / n) as i32, (input_position % n) as i32);

    (origin.0 - destination.0).abs() + (origin.1 - destination.1).abs()
}

#[cfg(test)]
mod tests {
    use super::get_manhattan_distance;

    #[test]
    fn test_get_manhattan_distance_sample_input_1() {
        let input = 1;
        let manhattan_distance = get_manhattan_distance(input);
        assert_eq!(0, manhattan_distance);
    }

    #[test]
    fn test_get_manhattan_distance_sample_input_2() {
        let input = 12;
        let manhattan_distance = get_manhattan_distance(input);
        assert_eq!(3, manhattan_distance);
    }

    #[test]
    fn test_get_manhattan_distance_sample_input_3() {
        let input = 23;
        let manhattan_distance = get_manhattan_distance(input);
        assert_eq!(2, manhattan_distance);
    }

    #[test]
    fn test_get_manhattan_distance_sample_input_4() {
        let input = 1024;
        let manhattan_distance = get_manhattan_distance(input);
        assert_eq!(31, manhattan_distance);
    }
}
