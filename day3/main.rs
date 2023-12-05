use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]

struct Number {
    value: u32,
    neighbors: HashSet<(i32, i32)>,
}

impl Number {
    pub fn new(value: u32, row: i32, col: i32) -> Self {
        let neighbors: HashSet<(i32, i32)> = HashSet::from([
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]);

        return Self { value, neighbors };
    }

    pub fn add_digit(&mut self, digit: u32, row: i32, col: i32) {
        self.value = self.value * 10 + digit;
        self.neighbors
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

fn part_one() {
    let file_path = "day3/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let mut sum = 0;

    let start_time = Instant::now();

    let mut symbols: HashSet<(i32, i32)> = HashSet::new();

    let mut current_number: Option<Number> = None;
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in file.lines().enumerate() {
        for (col, character) in line.unwrap().chars().enumerate() {
            if character.is_digit(10) {
                if current_number.is_none() {
                    current_number = Some(Number::new(
                        character.to_digit(10).unwrap(),
                        row as i32,
                        col as i32,
                    )); // if we dont have any current number, set it here
                } else if let Some(ref mut number) = current_number { // if we already have a number, then this digit is the next symbol of it
                    number.add_digit(character.to_digit(10).unwrap(), row as i32, col as i32) // add digit to current number
                }
            } else {
                if current_number.is_some() { // since we are done with this number, we push it to numbers vector
                    numbers.push(current_number.unwrap());
                }
                current_number = None; // and erase it from current number

                if character != '.' { // insert only special characters
                    symbols.insert((row as i32, col as i32));
                }
            }
        }
    }

    for number in &numbers {
        for neighbor in &number.neighbors {
            if neighbor.0 > 0 && neighbor.1 > 0 {
                if symbols.contains(&(neighbor.0, neighbor.1)) {
                    sum += number.value;
                    continue;
                }
            }
        }
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn part_two() {
    let file_path = "day3/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let mut sum = 0;

    let start_time = Instant::now();

    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    let mut current_number: Option<Number> = None;
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in file.lines().enumerate() {
        for (col, character) in line.unwrap().chars().enumerate() {
            if character.is_digit(10) {
                if current_number.is_none() {
                    current_number = Some(Number::new(
                        character.to_digit(10).unwrap(),
                        row as i32,
                        col as i32,
                    )); // if we dont have any current number, set it here
                } else if let Some(ref mut number) = current_number { // if we already have a number, then this digit is the next symbol of it
                    number.add_digit(character.to_digit(10).unwrap(), row as i32, col as i32) // add digit to current number
                }
            } else {
                if current_number.is_some() { // since we are done with this number, we push it to numbers vector
                    numbers.push(current_number.unwrap());
                }

                current_number = None; // and erase it from current number

                if character == '*' { // watch for gears
                    gears.insert((row as i32, col as i32), Vec::new()); // insert gears coordinates in gears hashmap
                }
            }
        }
    } // form our numbers vector, where numbers and their neighbors are stored

    for number in &numbers {
        for neighbor in &number.neighbors {
            if gears.contains_key(neighbor) {
                gears
                    .entry(*neighbor)
                    .and_modify(|neighbor_vaulues: &mut Vec<u32>| {
                        neighbor_vaulues.push(number.value);
                    });
            }
        }
    }

    gears.retain(|_, neighbors| neighbors.len() > 1); // filter out lonely parts without any gears

    for gear in gears.values() {
        let gear_power = gear.iter().nth(0).unwrap() * gear.iter().nth(1).unwrap();
        sum += gear_power;
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn main() {
    println!("Soving third day problem...");
    part_one();
    part_two();
}
