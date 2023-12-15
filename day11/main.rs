use std::io::prelude::*;
use std::io::SeekFrom; 
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: u64,
    y: u64,
}

fn calculate_distance(coordinate1: Coordinate, coordinate2: Coordinate) -> u64 {
    return coordinate1.x.abs_diff(coordinate2.x) + coordinate1.y.abs_diff(coordinate2.y);
}

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day11".to_string());

    let start_time = Instant::now();

    let mut galaxies: Vec<Coordinate> = Vec::new();

    let mut sum: u64 = 0;

    let mut rows_shift: usize = 0;
    let mut cols_shift: Vec<u32> = vec![0; input.by_ref().lines().count()];

    let _ = input.seek(SeekFrom::Start(0));

    let first_line = input.by_ref().lines().nth(0).unwrap().unwrap();

    for (col, char) in first_line.chars().enumerate() {
        let mut col_shift: u32 = cols_shift[if col > 0 { col - 1 } else { 0 }];

        if char == '.' {
            let mut is_expandable: bool = true;
            let _ = input.seek(SeekFrom::Start(0));

            for line in input.by_ref().lines() {
                if line.unwrap().chars().nth(col).unwrap() != '.' {
                    is_expandable = false;
                    break;
                }
            }
            if is_expandable {
                col_shift += 1;
            }
        }
        cols_shift[col] = col_shift;
    }

    let _ = input.seek(SeekFrom::Start(0));

    for (row, line) in input.by_ref().lines().enumerate() {
        let line = line.unwrap();
        if line.chars().filter(|character| *character != '.').count() > 0 {
            for (col, character) in line.chars().enumerate() {
                if character == '#' {
                    galaxies.push(Coordinate {
                        x: (col as u64 + cols_shift[col] as u64) as u64,
                        y: (row + rows_shift) as u64,
                    });
                }
            }
        } else {
            rows_shift += 1;
        }
    }

    for (index1, galaxy1) in galaxies.iter().enumerate() {
        for index2 in index1 + 1..galaxies.len() {
            sum += calculate_distance(*galaxy1, galaxies[index2]);
        }
    }

    print!(
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(), sum
    );
}

fn part_two() {
    let mut input = aoclib::file_reader::get_input("day11".to_string());

    let start_time = Instant::now();

    let mut galaxies: Vec<Coordinate> = Vec::new();

    let mut sum: u64 = 0;

    let mut rows_shift: usize = 0;
    let mut cols_shift: Vec<u64> = vec![0; input.by_ref().lines().count()];

    let _ = input.seek(SeekFrom::Start(0));

    let first_line = input.by_ref().lines().nth(0).unwrap().unwrap();

    for (col, char) in first_line.chars().enumerate() {
        let mut col_shift: u64 = cols_shift[if col > 0 { col - 1 } else { 0 }];

        if char == '.' {
            let mut is_expandable: bool = true;
            let _ = input.seek(SeekFrom::Start(0));

            for line in input.by_ref().lines() {
                if line.unwrap().chars().nth(col).unwrap() != '.' {
                    is_expandable = false;
                    break;
                }
            }

            if is_expandable {
                col_shift += 999999;
            }
        }
        cols_shift[col] = col_shift;
    }

    let _ = input.seek(SeekFrom::Start(0));

    for (row, line) in input.by_ref().lines().enumerate() {
        let line = line.unwrap();
        if line.chars().filter(|character| *character != '.').count() > 0 {
            for (col, character) in line.chars().enumerate() {
                if character == '#' {
                    galaxies.push(Coordinate {
                        x: (col as u64 + cols_shift[col]) as u64,
                        y: (row + rows_shift) as u64,
                    });
                }
            }
        } else {
            rows_shift += 999999; // we add 1_000_000 instead of previous 1
        }
    }

    for (index1, galaxy1) in galaxies.iter().enumerate() {
        for index2 in index1 + 1..galaxies.len() {
            sum += calculate_distance(*galaxy1, galaxies[index2]);
        }
    }

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(), sum
    );
}

fn main() {
    part_one();
    part_two();
}
