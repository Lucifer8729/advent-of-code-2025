use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut password = 0;
        let mut pointer = 50;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let mut chars = line.chars();
                    let direction = chars.next().unwrap();
                    let moves = chars.collect::<String>().parse::<i32>()?;

                    match direction {
                        'R' => {
                            pointer += moves;
                            pointer %= 100;
                            if pointer == 100 {
                                pointer = 0;
                            }
                        }
                        'L' => {
                            pointer -= moves;
                            pointer %= 100;
                            if pointer < 0 {
                                pointer = 100 + pointer;
                            }
                        }
                        _ => (),
                    }

                    if pointer == 0 {
                        password += 1;
                    }
                }
                _ => (),
            }
        }
        Ok(password)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut password = 0;
        let mut pointer = 50;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let mut chars = line.chars();
                    let direction = chars.next().unwrap();
                    let mut moves = chars.collect::<String>().parse::<i32>()?;

                    match direction {
                        'R' => {
                            while moves > 0 {
                                pointer += 1;
                                if pointer == 100 {
                                    pointer = 0;
                                }
                                if pointer == 0 {
                                    password += 1;
                                }
                                moves -= 1;
                            }
                        }
                        'L' => {
                            while moves > 0 {
                                if pointer == 0 {
                                    pointer = 100;
                                }
                                pointer -= 1;
                                if pointer == 0 {
                                    pointer = 100;
                                    password += 1;
                                }
                                moves -= 1;
                            }
                            if pointer == 100 {
                                pointer = 0;
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        Ok(password as usize)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
