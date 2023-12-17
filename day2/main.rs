use std::collections::HashMap;
use std::io::prelude::*;
use std::time::Instant;

fn is_game_possible(game: String) -> bool {
    let mut allowed_amounts: HashMap<String, usize> = HashMap::new();

    allowed_amounts.insert("g".to_string(), 13);
    allowed_amounts.insert("r".to_string(), 12);
    allowed_amounts.insert("b".to_string(), 14);

    let (_game_name, game_data) = game.split_once(": ").unwrap();

    let turns: Vec<_> = game_data.split("; ").collect();

    for turn in turns {
        let values: Vec<_> = turn.split(", ").collect();
        for value in values {
            let (amount, color) = value.split_once(" ").unwrap();
            if amount.parse::<usize>().unwrap()
                > *allowed_amounts
                    .get(&color.chars().next().unwrap().to_string())
                    .unwrap()
            {
                return false;
            }
        }
    }

    return true;
}

fn get_game_power(game: String) -> usize {
    let (_game_name, game_data) = game.split_once(": ").unwrap();

    let turns: Vec<_> = game_data.split("; ").collect();

    let mut maxmimum_amounts: HashMap<String, usize> = HashMap::new();

    maxmimum_amounts.insert("g".to_string(), 0);
    maxmimum_amounts.insert("r".to_string(), 0);
    maxmimum_amounts.insert("b".to_string(), 0);

    for turn in turns {
        let values: Vec<_> = turn.split(", ").collect();
        for value in values {
            let (amount, color) = value.split_once(" ").unwrap();
            if amount.parse::<usize>().unwrap()
                > *maxmimum_amounts
                    .get(&color.chars().next().unwrap().to_string())
                    .unwrap()
            {
                maxmimum_amounts
                    .entry(color.chars().next().unwrap().to_string())
                    .and_modify(|entry| {
                        *entry = amount.parse::<usize>().unwrap();
                    });
            }
        }
    }

    return *maxmimum_amounts.get(&"g".to_string()).unwrap()
        * *maxmimum_amounts.get(&"r".to_string()).unwrap()
        * *maxmimum_amounts.get(&"b".to_string()).unwrap();
}

fn part_one() {
    let input = aoclib::file_reader::get_input("day2".to_string());

    let mut possible_sum: usize = 0;

    let start_time = Instant::now();

    for (index, line) in input.lines().enumerate() {
        if is_game_possible(line.unwrap()) {
            possible_sum += index + 1
        }
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, possible_sum
    );
}

fn part_two() {
    let input = aoclib::file_reader::get_input("day2".to_string());

    let mut powers_sum: usize = 0;

    let start_time = Instant::now();

    for line in input.lines() {
        powers_sum += get_game_power(line.unwrap());
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;
    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, powers_sum
    );
}

fn main() {
    part_one();
    part_two();
}
