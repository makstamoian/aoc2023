use std::{collections::VecDeque, io::BufRead};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Downwards,
    Upwards,
}

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day16".to_string());

    let mut sum = 0;

    let mut contraption: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        let line_characters: Vec<char> = line.chars().collect();
        contraption.push(line_characters);
    }

    let mut next_vector: VecDeque<(u32, u32, Direction)> =
        VecDeque::from([(0, 0, Direction::Right)]);

    while next_vector.len() > 0 {
        let next = next_vector.pop_front();
        let next = next.unwrap();

        if contraption[next.1 as usize][next.0 as usize] == '.' {
            sum += 1;
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
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards))
                    } 
                },
                Direction::Left => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    } 

                },
                Direction::Upwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    } 
                },
                Direction::Downwards => {
                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                },
            };
        }

        if contraption[next.1 as usize][next.0 as usize] == '\\' {
            match next.2 {
                Direction::Left => {
                    if next.1 > 0 {
                        next_vector.push_back((next.0, next.1 - 1, Direction::Upwards))
                    } 
                },
                Direction::Right => {
                    if next.1 < (contraption.len() - 1) as u32 {
                        next_vector.push_back((next.0, next.1 + 1, Direction::Downwards))
                    } 
                },
                Direction::Downwards => {
                    if next.0 < (contraption[0].len() - 1) as u32 {
                        next_vector.push_back((next.0 + 1, next.1, Direction::Right))
                    } 
                },
                Direction::Upwards => {
                    if next.0 > 0 {
                        next_vector.push_back((next.0 - 1, next.1, Direction::Left))
                    }
                },
            };
        }
    }

    println!("{sum:#?}")
}

fn main() {
    part_one();
}
