use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut invalid_id_sum = 0;

        for line in reader.lines() {
            for id_range in line?.split(',') {
                let range = id_range.split("-").collect::<Vec<&str>>();
                let lower_bound = range.get(0).unwrap().parse::<usize>()?;
                let upper_bound = range.get(1).unwrap().parse::<usize>()?;

                let mut pointer = lower_bound;

                while pointer <= upper_bound {
                    let num_as_string = pointer.to_string();
                    if num_as_string.len() % 2 != 0 {
                        pointer += 1;
                        continue;
                    }

                    if is_valid_id(num_as_string.as_str()) {
                        invalid_id_sum += pointer;
                    }

                    pointer += 1;
                }
            }
        }

        Ok(invalid_id_sum)
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut invalid_id_sum = 0;

        for line in reader.lines() {
            for id_range in line?.split(',') {
                let range = id_range.split("-").collect::<Vec<&str>>();
                let lower_bound = range.get(0).unwrap().parse::<usize>()?;
                let upper_bound = range.get(1).unwrap().parse::<usize>()?;

                let mut pointer = lower_bound;

                while pointer <= upper_bound {
                    let num_as_string = pointer.to_string();

                    if is_valid_id_with_constraint(num_as_string.as_str()) {
                        invalid_id_sum += pointer;
                    }

                    pointer += 1;
                }
            }
        }

        Ok(invalid_id_sum)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn is_valid_id(id: &str) -> bool {
    let id_len = id.len();
    id[0..id_len / 2] == id[id_len / 2..id_len]
}

fn is_valid_id_with_constraint(id: &str) -> bool {
    let id_len = id.len();
    if id_len < 2 {
        return false;
    }

    let chars: Vec<char> = id.chars().collect();
    let mut pattern = chars.get(0).unwrap().to_string();
    let mut pointer = 1;

    while pointer < id_len {
        if pattern.len() > id_len / 2 {
            break;
        }

        let multiplication_factor = id_len / pattern.len();
        let expected_valid_id = pattern.repeat(multiplication_factor);
        if expected_valid_id == id {
            return true;
        }

        pattern += chars.get(pointer).unwrap().to_string().as_str();
        pointer += 1;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let is_valid = is_valid_id_with_constraint("38593859");
        assert_eq!(true, is_valid);
    }
}
