use std::io::Read;
use std::io::prelude::*;
use std::time::Instant;

fn is_sequence_zero (sequence: &Vec<i32>) -> bool {

    for sequence_item in sequence {
        if sequence_item != &0 {
            return false;
        }
    }
    return true;
}

fn get_shift_sequence(sequence: &Vec<i32>) -> Vec<i32> {
    let mut shift_sequence: Vec<i32> = Vec::new();
    for window in sequence.windows(2) {
        shift_sequence.push(window.iter().nth(1).unwrap() - window.iter().nth(0).unwrap());
    }
    return shift_sequence;
}

fn get_sequence_next_item (sequence: Vec<i32>) -> i32 {
    let mut next_item = 0;
    
    let mut shift_sequences: Vec<Vec<i32>> = Vec::from([sequence]);

    while !is_sequence_zero(shift_sequences.last().unwrap()) {
        shift_sequences.push(get_shift_sequence(shift_sequences.last().unwrap()))
    }

    for shift_sequence in &shift_sequences {
        next_item += shift_sequence.last().unwrap();
    }

    return next_item;
}

fn get_sequence_previous_item (sequence: Vec<i32>) -> i32 {
    let mut previous_item = 0;
    
    let mut shift_sequences: Vec<Vec<i32>> = Vec::from([sequence]);

    while !is_sequence_zero(shift_sequences.last().unwrap()) {
        shift_sequences.push(get_shift_sequence(shift_sequences.last().unwrap()))
    }

    for index in (1..shift_sequences.len()).rev() {
        previous_item = shift_sequences[index - 1][0] - previous_item;
    }

    return previous_item;
}

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day9".to_string());

    let start_time = Instant::now();

    let mut sum: i32 = 0;

    for line in input.by_ref().lines() {
        let line = line.unwrap();

        let sequnce: Vec<i32> = line.split_whitespace().map(|item| {
            item.parse::<i32>().unwrap()
        }).collect();

        sum += get_sequence_next_item(sequnce);
    }

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(), sum
    );
}


fn part_two() {
    let mut input = aoclib::file_reader::get_input("day9".to_string());

    let start_time = Instant::now();

    let mut sum: i32 = 0;

    for line in input.by_ref().lines() {
        let line = line.unwrap();

        let sequnce: Vec<i32> = line.split_whitespace().map(|item| {
            item.parse::<i32>().unwrap()
        }).collect();

        sum += get_sequence_previous_item(sequnce);
    }

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(), sum
    );
}

fn main () {
    part_one();
    part_two();
}