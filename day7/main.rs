use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time;
use std::time::Instant;

fn part_one() {
    let file_path = "day7/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let start_time = Instant::now();

    for line in file.by_ref().lines() {
        let line = line.unwrap();

        let (hand, bid) = line.split_once(" ").unwrap();

        println!("{:#?}, {:#?}", hand, bid);

        
    }

}

fn main () {
    part_one();
}