use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug)]

struct Number {
    value: u32,
    neighbors: HashSet<(i32, i32)>,
}

impl Number {
    pub fn new(value: u32, row: i32, col: i32) -> Self {
        let neighbors: HashSet<(i32, i32)> = HashSet::from([
            (row - 1, col - 1), // symbol on the top left   @ . .
            (row - 1, col), // symbol on the top middle     . @ .
            (row - 1, col + 1), // symbol on the top right  . . @

            (row, col - 1), // symbol to the left of given digit  @ . .
            (row, col + 1), // symbol to the right of given digit . . @
            
            (row + 1, col - 1), // symbol on the bottom left  @ . .
            (row + 1, col), // symbol on the bottom middle    . @ .
            (row + 1, col + 1), // symbol on the bottom right . . @
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
    let input = aoclib::file_reader::get_input("day03".to_string());

    let mut sum = 0;

    let start_time = Instant::now();

    let mut symbols: HashSet<(i32, i32)> = HashSet::new();

    let mut current_number: Option<Number> = None;
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in input.lines().enumerate() {
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
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn part_two() {
    let input = aoclib::file_reader::get_input("day03".to_string());

    let mut sum = 0;

    let start_time = Instant::now();

    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new();

    let mut current_number: Option<Number> = None;
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in input.lines().enumerate() {
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
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, sum
    );
}

fn main() {
    part_one();
    part_two();
}
