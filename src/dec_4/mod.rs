use crate::read_file;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Directions {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Directions {
    fn iterate() -> impl Iterator<Item = Directions> {
        use Directions::*;

        [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft]
            .iter()
            .copied()
    }

    fn new_index(index: (i32, i32), direction: Directions) -> (i32, i32) {
        let (x, y) = match direction {
            Directions::Up => (0, -1),
            Directions::UpRight => (1, -1),
            Directions::Right => (1, 0),
            Directions::DownRight => (1, 1),
            Directions::Down => (0, 1),
            Directions::DownLeft => (-1, 1),
            Directions::Left => (-1, 0),
            Directions::UpLeft => (-1, -1),
        };

        (index.0 + x, index.1 + y)
    }
}

pub fn day_four() {
    let input = read_file("src/dec_4/input.txt".to_string());
    let input = input.trim();

    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = Instant::now();
    let xmas_count = part_one(&matrix);
    let part_one_time = start.elapsed();

    let start = Instant::now();
    let x_mas_count = part_two(&matrix);
    let part_two_time = start.elapsed();

    println!("XMAS count: {} in {:?}", xmas_count, part_one_time);
    println!("X-MAS count: {} in {:?}", x_mas_count, part_two_time);
}

fn part_one(matrix: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let character = matrix[y][x];

            if character == 'X' {
                let index = (x as i32, y as i32);

                // Loop over every direction
                for direction in Directions::iterate() {
                    if get_character(&matrix, index, direction) == Some('M') {
                        let index = Directions::new_index(index, direction);

                        if get_character(&matrix, index, direction) == Some('A') {
                            let index = Directions::new_index(index, direction);

                            if get_character(&matrix, index, direction) == Some('S') {
                                xmas_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    xmas_count
}

fn part_two(matrix: &Vec<Vec<char>>) -> i32 {
    let mut x_mas_count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let character = matrix[y][x];

            if character == 'A' {
                let index = (x as i32, y as i32);

                let left_diag = (
                    get_character(&matrix, index, Directions::UpLeft),
                    get_character(&matrix, index, Directions::DownRight),
                );
                let right_diag = (
                    get_character(&matrix, index, Directions::UpRight),
                    get_character(&matrix, index, Directions::DownLeft),
                );

                if left_diag == (Some('M'), Some('S')) || left_diag == (Some('S'), Some('M')) {
                    if right_diag == (Some('M'), Some('S')) || right_diag == (Some('S'), Some('M'))
                    {
                        x_mas_count += 1;
                    }
                }
            }
        }
    }

    x_mas_count
}

/// Accepts the current matrix and index, and the direction to look in, and returns the character in that direction.
fn get_character(
    matrix: &Vec<Vec<char>>,
    index: (i32, i32),
    direction: Directions,
) -> Option<char> {
    let view_index = Directions::new_index(index, direction);

    // If the new index goes out of bounds, return none
    if view_index.0 >= 0 && view_index.1 >= 0 {
        if matrix.get(view_index.1 as usize).is_some() {
            if matrix[view_index.1 as usize]
                .get(view_index.0 as usize)
                .is_some()
            {
                return Some(matrix[view_index.1 as usize][view_index.0 as usize]);
            }
        }
    }

    return None;
}
