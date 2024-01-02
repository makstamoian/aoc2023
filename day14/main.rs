use std::io::prelude::*;

fn part_one() {
    let input = aoclib::file_reader::get_input("day14".to_string());
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let line = line.unwrap();
        matrix.push(line.chars().collect());
    }

    let mut current_rock: (u32, u32) = (0, 0);

    for col in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }
    for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }
    for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }
    for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }
    for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }for col in 0..matrix[0].len() {
        for row in (0..matrix.len()).rev() {
            if matrix[row][col] == 'O' {
                while row > 0 && matrix[row - 1][col] == '.' {
                    matrix[row][col] = '.';
                    matrix[row - 1][col] = 'O';
                }
            }
        }
    }

    for (row, line) in matrix.iter().enumerate() {
        for character in line {
            print!("{character}");
        }
        print!("\n");
        let load = matrix.len() - row;
        sum += load as u32 * (line.iter().filter(|character| *character == &'O').count()) as u32;
    }

    println!("{sum:?}");
}

fn main() {
    part_one();
}
