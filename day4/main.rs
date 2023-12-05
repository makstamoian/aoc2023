use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn part_one() {
    let file_path = "day4/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let start_time = Instant::now();

    let mut sum: u32 = 0;

    for line in file.lines() {
        let line = line.unwrap();
        let (_, card_informaion) = line.split_once(": ").unwrap();
        let (winning_numbers_string, our_numbers_string) = card_informaion.split_once(" | ").unwrap();
        let winning_numbers: Vec<u32> = winning_numbers_string.split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect();
        let our_numbers: Vec<u32> = our_numbers_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let mut card_sum: u32 = 0;

        for winning_number in &winning_numbers {
            if our_numbers.contains(winning_number) {
                if card_sum == 0 {
                    card_sum += 1;
                } else {
                    card_sum *= 2;
                }
            }
        }

        sum += card_sum;
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, sum
    );
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>
}

fn part_two () {
    let file_path = "day4/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let start_time = Instant::now();

    let mut sum: u32 = 0;
 
    let mut cards: Vec<Card> = Vec::new();

    for (index, line) in file.lines().enumerate() {
        let line = line.unwrap();
        let (_, card_informaion) = line.split_once(": ").unwrap();
        let (winning_numbers_string, our_numbers_string) = card_informaion.split_once(" | ").unwrap();
        let winning_numbers: Vec<u32> = winning_numbers_string.split_whitespace().map(|x| x.trim().parse::<u32>().unwrap()).collect();
        let our_numbers: Vec<u32> = our_numbers_string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        let mut overlapping_numbers_amount: u32 = 0;

        for winning_number in &winning_numbers {
            if our_numbers.contains(winning_number) {
                overlapping_numbers_amount += 1;
            }
        }

        for i in index + 1..(index + overlapping_numbers_amount as usize) {
            print!("{} ", i);
        }


    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn main () {
    println!("Soving fourth day problem...");
    part_one();
    part_two();
}