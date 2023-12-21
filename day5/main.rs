use std::io::prelude::*;
use std::time::Instant;

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day5".to_string());

    let start_time = Instant::now();
    let first_line = input.by_ref().lines().next().unwrap().unwrap();
    let (_, seeds_string) = first_line.split_once(": ").unwrap();

    let seeds: Vec<u32> = seeds_string
        .split(" ")
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect();

    let mut input_string: String = String::new();
    let _ = input.read_to_string(&mut input_string);

    let mut closest_location: i64 = i64::MAX;

    for seed in seeds {
        let mut location = seed as i64;

        for data in input_string.split("\n\n") {
            let data = data.trim().to_string();
            let (_, field_data) = data.split_once(":\n").unwrap();

            for line in field_data.split("\n") {
                let line = line.to_string();
                let range_data: Vec<i64> = line
                    .split(" ")
                    .map(|item| item.parse::<i64>().unwrap())
                    .collect();

                if location >= range_data[1] && location <= range_data[1] + range_data[2] {
                    if range_data[0] - range_data[1] > 0 {
                        location += range_data[0].abs_diff(range_data[1]) as i64;
                    } else {
                        location -= range_data[0].abs_diff(range_data[1]) as i64;
                    }
                    break;
                }
            }
        }

        if location < closest_location {
            closest_location = location;
        }
    }
    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(),
        closest_location
    );
}

fn part_two() {
    let mut input = aoclib::file_reader::get_input("day5".to_string());

    let start_time = Instant::now();
    let first_line = input.by_ref().lines().next().unwrap().unwrap();
    let (_, seeds_string) = first_line.split_once(": ").unwrap();

    let seeds_ranges: Vec<i64> = seeds_string
        .split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let mut seeds: Vec<i64> = Vec::new();

    for chunk in seeds_ranges.chunks(2) {
        for shift in 0..chunk[1] {
            seeds.push(chunk[0] + shift);
        }
    }

    let mut input_string: String = String::new();
    let _ = input.read_to_string(&mut input_string);

    let mut closest_location: i64 = i64::MAX;

    for mut location in seeds {
        for data in input_string.split("\n\n") {
            let data = data.trim().to_string();
            let (_, field_data) = data.split_once(":\n").unwrap();

            for line in field_data.split("\n") {
                let line = line.to_string();
                let range_data: Vec<i64> = line
                    .split(" ")
                    .map(|item| item.parse::<i64>().unwrap())
                    .collect();

                    if location >= range_data[1] && location <= range_data[1] + range_data[2] {
                        if range_data[0] as i32 - range_data[1] as i32 > 0 {
                            location += range_data[0].abs_diff(range_data[1]) as i64;
                        } else {
                            location -= range_data[0].abs_diff(range_data[1]) as i64;
                        }
                        break;
                    }
            }
        }

        if location < closest_location {
            closest_location = location;
        }
    }
    print!(
        "Finished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(),
        closest_location
    );
}

fn main() {
    part_one();
    part_two();
}
