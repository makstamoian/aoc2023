use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
    speeds: Vec<u32>
}

struct Race2 {
    time: u64,
    distance: u64,
    speeds: Vec<u64>
}

// struct Race {
//     time: u32,
//     distance: u32,
//     maximal_speed: Option<u32>,
//     minimal_speed: Option<u32>
// }

fn part_one () {
    let file_path = "day6/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let mut sum: u32 = 1;

    let mut races: Vec<Race> = Vec::new();

    let times_string = file.by_ref().lines().nth(0).unwrap().unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let distances_string = file.by_ref().lines().nth(1).unwrap().unwrap();

    let (_, mut times_string_values) = times_string.split_once(":").unwrap();
    let (_, mut distances_string_values) = distances_string.split_once(":").unwrap();
    times_string_values = times_string_values.trim();
    distances_string_values = distances_string_values.trim();

    

    let times: Vec<u32> = times_string_values.split_whitespace().map(|time| {
        time.parse::<u32>().unwrap()
    }).collect();

    let distances: Vec<u32> = distances_string_values.split_whitespace().map(|distance| {
        distance.parse::<u32>().unwrap()
    }).collect();

    for (race_index, time) in times.iter().enumerate() {
        races.push(Race { time: *time, distance: distances[race_index], speeds: Vec::new() });
        let distance = races[race_index].distance;
        let mut charging_time_and_speed: u32 = races[race_index].time;
        while charging_time_and_speed > 0 {
            if (f64::from(distance) / f64::from(charging_time_and_speed)) >= f64::from(time - charging_time_and_speed) {
                charging_time_and_speed -= 1;
            } else {
                races[race_index].speeds.push(charging_time_and_speed);                
                charging_time_and_speed -= 1;
            }
        }
    }

    for race in &races {
        sum *= (race.speeds[0] - race.speeds[race.speeds.len() - 1]) + 1;
    }

    let start_time = Instant::now();

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

    let mut sum: u64 = 1;


    let times_string = file.by_ref().lines().nth(0).unwrap().unwrap();
    let _ = file.seek(SeekFrom::Start(0));
    let distances_string = file.by_ref().lines().nth(1).unwrap().unwrap();

    let (_, mut times_string_values) = times_string.split_once(":").unwrap();
    let (_, mut distances_string_values) = distances_string.split_once(":").unwrap();
    times_string_values = times_string_values.trim();
    distances_string_values = distances_string_values.trim();

    let time_string_concatenated: String = times_string_values.split_whitespace().collect();
    let time = time_string_concatenated.parse::<u64>().unwrap();

    let distance_string_concatenated: String = distances_string_values.split_whitespace().collect();
    let distance = distance_string_concatenated.parse::<u64>().unwrap();

    let mut race: Race2 = Race2 { time, distance, speeds: Vec::new() };

    let distance = race.distance;
    let mut charging_time_and_speed: u64 = race.time;
    while charging_time_and_speed > 0 {
        if distance / charging_time_and_speed >= race.time - charging_time_and_speed {
            charging_time_and_speed -= 1;
        } else {
            race.speeds.push(charging_time_and_speed);                
            charging_time_and_speed -= 1;
        }
    }

    sum *= (race.speeds[0] - race.speeds[race.speeds.len() - 1]) + 1;


    let start_time = Instant::now();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        elapsed_time, sum
    );
}

fn main () {
    part_one();
    part_two();
}