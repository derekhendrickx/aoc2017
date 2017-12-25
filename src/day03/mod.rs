use day03::Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_manhattan_distance(input: i32) -> i32 {
    let n = (input as f32).sqrt().ceil() as i32;
    let mut spiral_memory = vec![vec![0; n as usize]; n as usize];
    let n_square = n * n;
    let mut direction = Left;
    let mut x = (n - 1) as usize;
    let mut y = x;
    let origin_y = (n / 2) as usize;

    if n % 2 == 0 {
        x = 0;
        y = x;
        direction = Right;
        spiral_memory[origin_y][origin_y - 1] = 1;
    } else {
        spiral_memory[origin_y][origin_y] = 1;
    }

    
    println!("{:?}", spiral_memory);

    for value in (1..n_square).rev() {
        println!("{:?}", spiral_memory);
        println!("{}", value + 1);
        spiral_memory[y][x] = value + 1;

        match direction {
            Up => {
                if y == 0 || spiral_memory[y - 1][x] > 0 {
                    direction = Right;
                }
            }
            Down => {
                if y == (n - 1) as usize || spiral_memory[y + 1][x] > 0 {
                    direction = Left;
                }
            }
            Left => {
                if x == 0 || spiral_memory[y][x - 1] > 0 {
                    direction = Up;
                }
            }
            Right => {
                if x == (n - 1) as usize || spiral_memory[y][x + 1] > 0 {
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

    println!("{:?}", spiral_memory);

    let flatten_spiral = spiral_memory
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .collect::<Vec<i32>>();
    let origin_position = flatten_spiral.iter().position(|&value| value == 1).unwrap();
    let origin = ((origin_position as i32) / n, (origin_position as i32) % n);
    let input_position = flatten_spiral
        .iter()
        .position(|&value| value == input)
        .unwrap();
    let destination = ((input_position as i32) / n, (input_position as i32) % n);

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
