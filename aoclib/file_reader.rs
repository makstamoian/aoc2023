use std::fs::File;
use std::io::BufReader;

pub fn get_input (day: String) -> BufReader<File> {
    let file_path = format!("{}/input.txt", day);
    let file = File::open(file_path).unwrap();
    return BufReader::new(file);
}