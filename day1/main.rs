use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::BTreeMap;

fn get_numeric_characters(string: String) -> String {
    let mut output: String = "".to_string();
    for character in string.chars() {
        if character.is_numeric() {
            output.push(character);
        }
    }
    return output.to_string();
}

fn get_numeric_characters_or_spellings(string: String) -> String {

    let mut spelling_aliases: HashMap<String, u32> = HashMap::new();

    spelling_aliases.insert("one".to_string(), 1);
    spelling_aliases.insert("two".to_string(), 2);
    spelling_aliases.insert("three".to_string(), 3);
    spelling_aliases.insert("four".to_string(), 4);
    spelling_aliases.insert("five".to_string(), 5);
    spelling_aliases.insert("six".to_string(), 6);
    spelling_aliases.insert("seven".to_string(), 7);
    spelling_aliases.insert("eight".to_string(), 8);
    spelling_aliases.insert("nine".to_string(), 9);

    let mut output: String = "".to_string();

    let mut indexed_characters: BTreeMap<u32, u32> = BTreeMap::new();

    for spelling in spelling_aliases.keys() {
        let occurrences: Vec<_> = string.match_indices(spelling).map(|(i, _)|i).collect();
        if occurrences.len() > 0 {
            indexed_characters.insert(occurrences[0] as u32, *spelling_aliases.get(spelling).unwrap());
            indexed_characters.insert(occurrences[occurrences.len() - 1] as u32, *spelling_aliases.get(spelling).unwrap());
        }
    }

    for (index, character) in string.chars().enumerate() {
        if character.is_numeric() {
            indexed_characters.insert(index as u32, character.to_digit(10).unwrap());
        }
    }

    output.push_str(&indexed_characters.values().next().unwrap().to_string());
    output.push_str(&indexed_characters.values().next_back().unwrap().to_string());

    return output;
}

fn first_part() {
    let file_path = "day1/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);
    let mut sum: u32 = 0;

    let start_time = Instant::now();

    for line in file.lines() {
        let mut calibration_string: String = "".to_string();
        let only_numeric_characters = get_numeric_characters(line.unwrap());
        calibration_string.push(only_numeric_characters.chars().nth(0).unwrap());
        calibration_string.push(only_numeric_characters.chars().nth(only_numeric_characters.len() - 1).unwrap());
        let calibration_value: u32 = calibration_string.parse::<u32>().unwrap();
        sum += calibration_value;
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!("\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m", elapsed_time, sum);
}

fn second_part() {
    let file_path = "day1/input.txt";
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);
    let mut sum: u32 = 0;

    let start_time = Instant::now();

    for line in file.lines() {
        sum += get_numeric_characters_or_spellings(line.unwrap()).parse::<u32>().unwrap();
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!("\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n", elapsed_time, sum);
}

fn main() {
    first_part();
    second_part();
}