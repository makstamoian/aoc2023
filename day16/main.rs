use std::{collections::{VecDeque, HashSet}, io::BufRead, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    Right,
    Left,
    Downwards,
    Upwards,
}

fn energized_for_current_config(
    contraption: &Vec<Vec<char>>,
    starting_beam: (u32, u32, Direction),
) -> u32 {
    let mut energized_beams: u32 = 0;
    let mut next_vector: VecDeque<(u32, u32, Direction)> = VecDeque::from([starting_beam]);
    let mut visited: HashSet<(u32, u32, Direction)> = HashSet::new();
    let mut visited_once: HashSet<(u32, u32)> = HashSet::new();

    while next_vector.len() > 0 {
        let next = next_vector.pop_back();
        let next = next.unwrap();

        if visited.contains(&next) {
            continue;
        }
        if !visited_once.contains(&(next.0, next.1)) {
            energized_beams += 1;
            visited_once.insert((next.0, next.1));
        }
        visited.insert(next);

        if contraption[next.1 as usize][next.0 as usize] == '.' {
            
            match next.2 {
                Direction::Right => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, next.2))
                    }
                }
                Direction::Left => {
                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, next.2))
                    }
                }
                Direction::Downwards => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, next.2))
                    }
                }
                Direction::Upwards => {
                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, next.2))
                    }
                }
            };
        }

        if contraption[next.1 as usize][next.0 as usize] == '|' {
            
            match next.2 {
                Direction::Right => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    }

                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards))
                    }
                }
                Direction::Left => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    }

                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards))
                    }
                }
                Direction::Downwards => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, next.2))
                    }
                }
                Direction::Upwards => {
                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, next.2))
                    }
                }
            };
        }

        if contraption[next.1 as usize][next.0 as usize] == '-' {
            
            match next.2 {
                Direction::Downwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    }

                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                }
                Direction::Upwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    }

                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                }
                Direction::Left => {
                    if next.0 > 0 as u32 {
                        next_vector.push_back((next.0 - 1, next.1, next.2))
                    }
                }
                Direction::Right => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, next.2))
                    }
                }
            };
        }

        if contraption[next.1 as usize][next.0 as usize] == '/' {
            
            match next.2 {
                Direction::Right => {
                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards));
                    }
                }
                Direction::Left => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    }
                }
                Direction::Upwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    }
                }
                Direction::Downwards => {
                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                }
            };
        }

        if contraption[next.1 as usize][next.0 as usize] == '\\' {
            
            match next.2 {
                Direction::Left => {
                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards))
                    }
                }
                Direction::Right => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    }
                }
                Direction::Downwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    }
                }
                Direction::Upwards => {
                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                }
            };
        }
    }
    return energized_beams;
}

fn get_possible_starting_beams(contraption: &Vec<Vec<char>>) -> HashSet<(u32, u32, Direction)> {
    let mut possible_starting_beams: HashSet<(u32, u32, Direction)> = HashSet::new();

    let corner_beams: HashSet<(u32, u32, Direction)> = HashSet::from([ // corners
        // 8 of them because we have to track the directions also
        (0, 0, Direction::Right),
        (0, 0, Direction::Downwards),
        (0, (contraption.len() - 1) as u32, Direction::Right),
        (0, (contraption.len() - 1) as u32, Direction::Upwards),
        ((contraption[0].len() - 1) as u32, 0, Direction::Left),
        ((contraption[0].len() - 1) as u32, 0, Direction::Downwards),
        (
            (contraption[0].len() - 1) as u32,
            (contraption.len() - 1) as u32,
            Direction::Left,
        ),
        (
            (contraption[0].len() - 1) as u32,
            (contraption.len() - 1) as u32,
            Direction::Upwards,
        ),
    ]);

    for y_index in 1..contraption.len() - 2 {
        // all exept first and the last ones
        if contraption[y_index][0] == '.' {
            possible_starting_beams.insert((0, y_index as u32, Direction::Upwards));
            possible_starting_beams.insert((0, y_index as u32, Direction::Downwards));
            possible_starting_beams.insert((0, y_index as u32, Direction::Right));
        }
        if contraption[y_index][contraption[0].len() - 2] == '.' {
            possible_starting_beams.insert((
                (contraption[0].len() - 2) as u32,
                y_index as u32,
                Direction::Upwards,
            ));
            possible_starting_beams.insert((
                (contraption[0].len() - 2) as u32,
                y_index as u32,
                Direction::Downwards,
            ));
            possible_starting_beams.insert((
                (contraption[0].len() - 2) as u32,
                y_index as u32,
                Direction::Left,
            ));
        }
    }

    for x_index in 1..contraption[0].len() - 2 {
        // all exept first and the last ones
        if contraption[0][x_index] == '.' {
            possible_starting_beams.insert((x_index as u32, 0 as u32, Direction::Downwards));
            possible_starting_beams.insert((x_index as u32, 0 as u32, Direction::Left));
            possible_starting_beams.insert((x_index as u32, 0 as u32, Direction::Right));
        }
        if contraption[contraption.len() - 2][x_index] == '.' {
            possible_starting_beams.insert((
                x_index as u32,
                (contraption.len() - 2) as u32,
                Direction::Upwards,
            ));
            possible_starting_beams.insert((
                x_index as u32,
                (contraption.len() - 2) as u32,
                Direction::Downwards,
            ));
            possible_starting_beams.insert((
                x_index as u32,
                (contraption.len() - 2) as u32,
                Direction::Left,
            ));
        }
    }

    possible_starting_beams.extend(corner_beams);

    // Here we will do some calculations...

    return possible_starting_beams;
}
fn part_one() {
    let input = aoclib::file_reader::get_input("day16".to_string());

    let start_time = Instant::now();

    let mut contraption: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        let line_characters: Vec<char> = line.chars().collect();
        contraption.push(line_characters);
    }

    let sum = energized_for_current_config(&contraption, (0, 0, Direction::Right));

    print!(
        "Finished part 1 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m",
        start_time.elapsed(),
        sum
    );
}

fn part_two() {
    let input = aoclib::file_reader::get_input("day16".to_string());

    let start_time = Instant::now();

    let mut contraption: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        let line_characters: Vec<char> = line.chars().collect();
        contraption.push(line_characters);
    }

    let possible_starting_beams: HashSet<(u32, u32, Direction)> =
        get_possible_starting_beams(&contraption);

    let mut max_energized_beams: u32 = 0;

    for starting_beam in possible_starting_beams {
        let energized_for_this_beam = energized_for_current_config(&contraption, starting_beam);
        if energized_for_this_beam > max_energized_beams {
            max_energized_beams = energized_for_this_beam;
        }
    }

    print!(
        "\nFinished part 2 in: \x1b[1m{:#?}\x1b[0m with answer: \x1b[1m{:#?}\x1b[0m\n",
        start_time.elapsed(),
        max_energized_beams
    );
}

fn main() {
    part_one();
    part_two();
}
