use std::{io::prelude::*, time::Instant};

fn tilt_up(matrix: &mut Vec<Vec<char>>) {
    for col in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            if matrix[row][col] == 'O' {
                let mut shift_row = row;
                while shift_row > 0 && matrix[shift_row - 1][col] == '.' {
                    matrix[shift_row][col] = '.';
                    matrix[shift_row - 1][col] = 'O';
                    shift_row -= 1;
                }
            }
        }
    }
}

fn tilt_down(matrix: &mut Vec<Vec<char>>) {
    for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                let mut shift_row = row;
                while shift_row < matrix.len() - 1 && matrix[shift_row + 1][col] == '.' {
                    matrix[shift_row][col] = '.';
                    matrix[shift_row + 1][col] = 'O';
                    shift_row += 1;
                }
            }
        }
    }
}

fn tilt_right(matrix: &mut Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in (0..matrix[0].len()).rev() {
            if matrix[row][col] == 'O' {
                let mut shift_col: usize = col;
                while shift_col < matrix[0].len() - 1 && matrix[row][shift_col + 1] == '.' {
                    matrix[row][shift_col] = '.';
                    matrix[row][shift_col + 1] = 'O';
                    shift_col += 1;
                }
            }
        }
    }
}

fn tilt_left(matrix: &mut Vec<Vec<char>>) {
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'O' {
                let mut shift_col: usize = col;
                while shift_col > 0 && matrix[row][shift_col - 1] == '.' {
                    matrix[row][shift_col] = '.';
                    matrix[row][shift_col - 1] = 'O';
                    shift_col -= 1;
                }
            }
        }
    }
}

fn part_one() {
    let input = aoclib::file_reader::get_input("day14".to_string());
    let start_time = Instant::now();
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }

    tilt_up(&mut matrix);

    for (row, line) in matrix.iter().enumerate() {
        let load = matrix.len() - row;
        sum += load as u32 * (line.iter().filter(|character| *character == &'O').count()) as u32;
    }

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(),
        sum
    );
}

fn part_two() {
    let input = aoclib::file_reader::get_input("day14".to_string());
    let start_time = Instant::now();
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }

    for _ in 0..1000 {
        tilt_up(&mut matrix);
        tilt_left(&mut matrix);
        tilt_down(&mut matrix);
        tilt_right(&mut matrix);
    }

    for (row, line) in matrix.iter().enumerate() {
        let load = matrix.len() - row;
        sum += load as u32 * (line.iter().filter(|character| *character == &'O').count()) as u32;
    }

    print!(
        "Finished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(),
        sum
    );
}

fn main() {
    part_one();
    part_two();
}
