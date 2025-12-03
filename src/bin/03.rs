use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut joltage = 0;
        for line in reader.lines() {
            let line = line?;

            if line.len() <= 2 {
                let voltage = line.parse::<i32>()?;
                joltage += voltage;
                continue;
            }

            let digits: Vec<char> = line.chars().collect_vec();

            let mut first_digit = digits.get(0).unwrap();
            let mut first_digit_idx = 0;

            let mut largest = first_digit.to_string().parse::<i32>()?;
            for (idx, d) in digits.iter().enumerate() {
                if idx == line.chars().count() - 1 {
                    break;
                }
                let digit = d.to_string().parse::<i32>()?;
                if digit > largest {
                    first_digit = &d;
                    largest = digit;
                    first_digit_idx = idx;
                }
            }

            let mut second_digit = digits.get(first_digit_idx + 1).unwrap();
            largest = second_digit.to_string().parse::<i32>()?;
            for (idx, d) in digits.iter().enumerate() {
                if idx <= first_digit_idx {
                    continue;
                }

                let digit = d.to_string().parse::<i32>()?;
                if digit > largest {
                    second_digit = &d;
                    largest = digit;
                }
            }

            let voltage = format!("{}{}", first_digit, second_digit);
            joltage += voltage.parse::<i32>()?;
        }

        Ok(joltage as usize)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u128> {
        let mut joltage = 0;
        for line in reader.lines() {
            let line = line?;
            let digits: Vec<char> = line.chars().collect();
            let length = digits.len();

            let mut voltage: Vec<char> = Vec::new();
            let mut limit: i32 = 11;
            let mut digit_idx = 0;
            while limit >= 0 {
                let (digit, idx) =
                    largest_from_and_before(digits.clone(), digit_idx, length - limit as usize);

                voltage.push(digit);
                limit -= 1;
                digit_idx = idx + 1;
            }

            joltage += voltage.iter().join("").parse::<u128>()?;
        }

        Ok(joltage as u128)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn largest_from_and_before(digits: Vec<char>, start_idx: usize, end_idx: usize) -> (char, usize) {
    let mut idx = start_idx;
    let mut largest = digits[idx].to_string().parse::<i32>().unwrap();
    let mut largest_idx = start_idx;
    while idx < end_idx {
        let digit = digits[idx].to_string().parse::<i32>().unwrap();
        if digit > largest {
            largest = digit;
            largest_idx = idx;
        }

        idx += 1;
    }

    (digits[largest_idx], largest_idx)
}
