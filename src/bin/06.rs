use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut operands: Vec<Vec<usize>> = Vec::new();
        let mut operations: Vec<char> = Vec::new();
        for line in reader.lines() {
            let line = line?;

            if line.contains("+") || line.contains("*") {
                for operator in line.split_whitespace() {
                    operations.push(operator.chars().next().unwrap());
                }
                continue;
            }

            let mut nums: Vec<usize> = Vec::new();
            for num in line.split_whitespace() {
                nums.push(num.parse::<usize>()?);
            }
            operands.push(nums);
        }

        let mut final_solution = 0;
        for i in 0..operations.len() {
            match operations.get(i).unwrap() {
                '+' => {
                    let mut solution = 0;
                    for operand in &operands {
                        solution += operand[i];
                    }
                    final_solution += solution;
                }
                '*' => {
                    let mut solution = 1;
                    for operand in &operands {
                        solution *= operand[i];
                    }
                    final_solution += solution;
                }
                _ => continue,
            }
        }

        Ok(final_solution)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut operands: Vec<String> = Vec::new();
        let mut operations: Vec<char> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line.contains("+") || line.contains("*") {
                for op in line.chars() {
                    operations.push(op);
                }
                continue;
            }

            operands.push(line);
        }

        let mut sum_equations: Vec<(&str, &str, &str, &str)> = Vec::new();
        let mut mul_equations: Vec<(&str, &str, &str, &str)> = Vec::new();

        let mut idx = 0;
        while idx < operations.len() {
            let mut end = idx + 1;
            if end > operations.len() {
                break;
            }
            if end == operations.len() {
                end = operands[0].len() + 1
            }
            while end < operations.len() && operations[end] != '+' && operations[end] != '*' {
                end += 1;
            }

            let nums: (&str, &str, &str, &str) = (
                operands[0].get(idx..end - 1).unwrap(),
                operands[1].get(idx..end - 1).unwrap(),
                operands[2].get(idx..end - 1).unwrap(),
                operands[3].get(idx..end - 1).unwrap(),
            );

            match operations[idx] {
                '+' => sum_equations.push(nums),
                '*' => mul_equations.push(nums),
                _ => (),
            }
            idx = end;
        }

        let mut solution = 0;
        for (q1, q2, q3, q4) in sum_equations {
            let nums = get_digits(Vec::from([q1, q2, q3, q4]));
            let mut sum = 0;
            for num in nums {
                sum += num;
            }
            solution += sum;
        }
        for (q1, q2, q3, q4) in mul_equations {
            let nums = get_digits(Vec::from([q1, q2, q3, q4]));
            let mut mul = 1;
            for num in nums {
                mul *= num;
            }
            solution += mul;
        }

        Ok(solution)
    }

    // assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn get_digits(input: Vec<&str>) -> Vec<usize> {
    let mut nums: Vec<usize> = Vec::new();
    for i in 0..input[0].len() {
        let mut digit = String::new();
        input.iter().for_each(|val| {
            digit += val.chars().nth(i).unwrap().to_string().as_str();
        });
        nums.push(digit.trim().parse::<usize>().unwrap());
    }
    nums
}
