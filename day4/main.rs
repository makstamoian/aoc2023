use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;
use std::collections::HashMap;

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
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );

}


fn part_two () {
    let file_path = "day4/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let mut sum: u32 = 0;

    let start_time = Instant::now();
 
    let mut cards_copies: HashMap<u32, u32> = HashMap::new();

    for index in 0..file.by_ref().lines().count() {
        cards_copies.insert(index as u32, 1);
    }

    let _ = file.seek(SeekFrom::Start(0));

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

        let current_card_copies: u32 = *cards_copies.get(&(index as u32)).unwrap();

        for i in (index + 1)..overlapping_numbers_amount as usize + index + 1 {
            cards_copies.entry(i as u32).and_modify(|copies| {
                *copies += 1 * current_card_copies;
            });
        }
    }

    for (_, card_copies_amount) in cards_copies {
        sum += card_copies_amount;
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, sum
    );
}

fn main () {
    println!("Soving fourth day problem...");
    part_one();
    part_two();
}