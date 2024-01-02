use std::{io::prelude::*, time::Instant};

fn part_one() {
    let input = aoclib::file_reader::get_input("day14".to_string());
    let start_time = Instant::now();
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }

    fn max_out_current_column(col: usize, matrix: &mut Vec<Vec<char>>) {
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

    for col in 0..matrix[0].len() {
        max_out_current_column(col, &mut matrix);
    }

    for (row, line) in matrix.iter().enumerate() {
        let load = matrix.len() - row;
        sum += load as u32 * (line.iter().filter(|character| *character == &'O').count()) as u32;
    }

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(), sum
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

    fn tilt_up(col: usize, matrix: &mut Vec<Vec<char>>) {
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

    fn tilt_down(col: usize, matrix: &mut Vec<Vec<char>>) {
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

    for col in 0..matrix[0].len() {
        tilt_up(col, &mut matrix);
    }

    for (row, line) in matrix.iter().enumerate() {
        let load = matrix.len() - row;
        sum += load as u32 * (line.iter().filter(|character| *character == &'O').count()) as u32;
    }

    print!(
        "Finished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(), sum
    );

}

fn main() {
    part_one();
}
