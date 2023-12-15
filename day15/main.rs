use std::io::BufRead;
use std::time::Instant;

fn calculate_hash(character: char) -> u32 {
    return character as u32;
}

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day15".to_string());
    let start_time = Instant::now();


    let mut initialization_sequence: String = String::new();
    let _ = input.read_line(&mut initialization_sequence);
    let initialization_items: Vec<String> = initialization_sequence
        .split(",")
        .map(str::to_string)
        .collect();

    let mut sum: u32 = 0;

    for item in initialization_items {
        let mut item_sum: u32 = 0;
        for character in item.chars() {
            item_sum += calculate_hash(character);
            item_sum *= 17;
            item_sum = item_sum % 256;
        }
        sum += item_sum;
    }

    print!(
        "\nFinished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(), sum
    );
}

fn part_two() {
    let mut input = aoclib::file_reader::get_input("day15".to_string());

    let start_time = Instant::now();

    let mut initialization_sequence: String = String::new();
    let _ = input.read_line(&mut initialization_sequence);
    let initialization_items: Vec<String> = initialization_sequence
        .split(",")
        .map(str::to_string)
        .collect();

    let mut sum: u32 = 0;

    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

    for item in initialization_items { 
        let mut related_box: u32 = 0;
        let mut label: String = String::new();
        let mut focal_length: u32 = 0;
        let mut operation: char = ' ';
        
        for character in item.chars() {
            if character.is_alphabetic() {
                label.push(character);
                
                related_box += calculate_hash(character);
                related_box *= 17;
                related_box = related_box % 256;

                continue;
            }
            if character.is_digit(10) {
                focal_length = character.to_string().parse::<u32>().unwrap();
                continue;
            }
            operation = character;
        }

        // println!("{label:?}: {related_box:?}, {focal_length:?}, {operation:?}, {:#?}", boxes[related_box as usize]);

        if boxes[related_box as usize].iter().filter(|lens| { lens.0 == label }).count() > 0 {
            let index = boxes[related_box as usize].iter().position(|lens| { lens.0 == label }).unwrap();

            if operation == '-' {
                boxes[related_box as usize].remove(index);
            }

            if operation == '=' {
                boxes[related_box as usize][index].1 = focal_length;
            }

        } else {
            if operation == '=' {
                boxes[related_box as usize].push((label, focal_length));
            }
        }

    }

    // println!("{:#?}", boxes)

    for (box_index, box_container) in boxes.iter().enumerate() {
        for lens in box_container.iter().enumerate() {
            let lens_sum = (box_index + 1) * (lens.0 + 1) * (lens.1.1 as usize);
            sum += lens_sum as u32;
        }
    }

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(), sum
    );
}

fn main() {
    part_one();
    part_two();
}
