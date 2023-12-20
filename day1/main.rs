use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::prelude::*;
use std::time::Instant;

fn get_numeric_characters(string: String) -> u32 {
    let mut left_index = 0;
    let mut right_index = string.len() - 1;
    let mut first_number: u32 = 0;
    let mut second_number: u32 = 0;

    while (first_number == 0 || second_number == 0)
        && (left_index < string.len() && right_index > 0)
    {
        if first_number == 0 {
            first_number = string
                .chars()
                .nth(left_index)
                .unwrap()
                .to_digit(10)
                .unwrap_or(0);
            left_index += 1;
        }

        if second_number == 0 {
            second_number = string
                .chars()
                .nth(right_index)
                .unwrap()
                .to_digit(10)
                .unwrap_or(0);
            right_index -= 1;
        }
    }

    return first_number * 10 + second_number;
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
        let occurrences: Vec<_> = string.match_indices(spelling).map(|(i, _)| i).collect();
        if occurrences.len() > 0 {
            indexed_characters.insert(
                occurrences[0] as u32,
                *spelling_aliases.get(spelling).unwrap(),
            );
            indexed_characters.insert(
                occurrences[occurrences.len() - 1] as u32,
                *spelling_aliases.get(spelling).unwrap(),
            );
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

fn part_one() {
    let input = aoclib::file_reader::get_input("day1".to_string());

    let mut sum: u32 = 0;

    let start_time = Instant::now();

    for line in input.lines() {
        sum += get_numeric_characters(line.unwrap());
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn part_two() {
    let input = aoclib::file_reader::get_input("day1".to_string());

    let mut sum: u32 = 0;

    let start_time = Instant::now();

    for line in input.lines() {
        sum += get_numeric_characters_or_spellings(line.unwrap())
            .parse::<u32>()
            .unwrap();
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
