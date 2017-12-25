enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_manhattan_distance(n: i32, input: i32) -> i32 {
    let (x1, y1) = (0, 0);
    let mut spiral_memory = vec![vec![0; n as usize]; n as usize];
    let n_square = n * n;
    let mut direction = Direction::Right;
    let mut x = 0 as usize;
    let mut y = 0 as usize;

    if n_square % 2 == 0 {
        spiral_memory[y][x] = n_square;
    } else {
        direction = Direction::Left;
        x = (n - 1) as usize;
        y = (n - 1) as usize;
        spiral_memory[y][x] = n_square;
    }

    for value in (1..n_square + 1).rev() {
        let row_value = spiral_memory[y][x];
        if row_value == 0 {
            spiral_memory[y][x] = value;
        }

        match direction {
            Direction::Up => {
                if y == 0 || spiral_memory[y - 1][x] > 0 {
                    direction = Direction::Right;
                }
            },
            Direction::Down => {
                if spiral_memory[y + 1][x] > 0 {
                    direction = Direction::Left;
                }
            },
            Direction::Left => {
                if x == 0 || spiral_memory[y][x - 1] > 0 {
                    direction = Direction::Up;
                }
            },
            Direction::Right => {
                if x == 4 || spiral_memory[y][x + 1] > 0 {
                    direction = Direction::Down;
                }
            },
        }

        match direction {
            Direction::Up => {
                y -= 1;
            },
            Direction::Down => {
                y += 1;
            },
            Direction::Left => {
                x -= 1;
            },
            Direction::Right => {
                x += 1;
            },
        }
    }
    println!("{:?}", spiral_memory);

    // Manhattan distance
    // (x1 - x2) + (y1 - y2)
    let x2 = x as i32;
    let y2 = y as i32;
    let distance: i32 = (x1 - x2) + (y1 - y2);

    distance.abs()
}

#[cfg(test)]
mod tests {
    use super::get_manhattan_distance;

    #[test]
    fn test_get_manhattan_distance_sample_input_1() {
        let input = 1;
        let manhattan_distance = get_manhattan_distance(5, input);
        assert_eq!(0, manhattan_distance);
    }

    #[test]
    fn test_get_manhattan_distance_sample_input_2() {
        let input = 12;
        let manhattan_distance = get_manhattan_distance(5, input);
        assert_eq!(3, manhattan_distance);
    }
}
