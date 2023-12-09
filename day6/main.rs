use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

fn part_one () {
    let file_path = "day6/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let start_time = Instant::now();

    let mut sum: u32 = 1;

    let times_string = file.by_ref().lines().nth(0).unwrap().unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let distances_string = file.by_ref().lines().nth(1).unwrap().unwrap();

    let (_, mut times_string_values) = times_string.split_once(":").unwrap();
    let (_, mut distances_string_values) = distances_string.split_once(":").unwrap();
    times_string_values = times_string_values.trim();
    distances_string_values = distances_string_values.trim();

    let times: Vec<f32> = times_string_values.split_whitespace().map(|time| {
        time.parse::<f32>().unwrap()
    }).collect();

    let distances: Vec<f32> = distances_string_values.split_whitespace().map(|distance| {
        distance.parse::<f32>().unwrap()
    }).collect();

    for (race_index, time) in times.iter().enumerate() {
        let distance = distances[race_index];
        
        let (alpha, beta) = (
            (- time + f32::sqrt((time * time) - (4.0 * distance))) / -2.0,
            (- time - f32::sqrt((time * time) - (4.0 * distance))) / -2.0
        );
    
        let ways_count = beta.floor() as u32 - (
            if alpha.floor() == alpha {
                alpha.floor() as u32 + 1
            } else {
                alpha.floor() as u32
            }
        );

        sum *= ways_count
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        elapsed_time, sum
    );
}

fn part_two () {
    let file_path = "day6/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let start_time = Instant::now();

    let times_string = file.by_ref().lines().nth(0).unwrap().unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let distances_string = file.by_ref().lines().nth(1).unwrap().unwrap();

    let (_, mut times_string_values) = times_string.split_once(":").unwrap();
    let (_, mut distances_string_values) = distances_string.split_once(":").unwrap();
    times_string_values = times_string_values.trim();
    distances_string_values = distances_string_values.trim();

    let time_string_concatenated: String = times_string_values.split_whitespace().collect();
    let time = time_string_concatenated.parse::<f64>().unwrap();

    let distance_string_concatenated: String = distances_string_values.split_whitespace().collect();
    let distance = distance_string_concatenated.parse::<f64>().unwrap();

    let (alpha, beta) = (
        (- time + f64::sqrt((time * time) - (4.0 * distance))) / -2.0,
        (- time - f64::sqrt((time * time) - (4.0 * distance))) / -2.0
    );

    let ways_count = beta.floor() as u64 - (
        if alpha.floor() == alpha {
            alpha.floor() as u64 + 1
        } else {
            alpha.floor() as u64
        }
    );

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, ways_count
    );
}

fn main () {
    part_one();
    part_two();
}