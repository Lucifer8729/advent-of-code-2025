use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // fn part1<R: BufRead>(reader: R) -> Result<usize> {
    //     let mut fresh_id = 0;
    //     let mut fresh_ids: HashSet<usize> = HashSet::new();
    //     for line in reader.lines() {
    //         let line = line?;
    //
    //         if line.contains('-') {
    //             let range: Vec<&str> = line.split('-').collect();
    //             let mut start = range[0].parse::<usize>()?;
    //             let end = range[1].parse::<usize>()?;
    //
    //             while start <= end {
    //                 fresh_ids.insert(start);
    //                 start += 1
    //             }
    //
    //             continue;
    //         }
    //
    //         if line == "" {
    //             continue;
    //         }
    //
    //         let id = line.parse::<usize>()?;
    //         if fresh_ids.contains(&id) {
    //             fresh_id += 1;
    //         }
    //     }
    //
    //     Ok(fresh_id)
    // }
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut fresh_id = 0;
        let mut all_ranges: Vec<(usize, usize)> = Vec::new();

        'lines: for line in reader.lines() {
            let line = line?;

            if line.contains('-') {
                let range: Vec<&str> = line.split('-').collect();
                let start = range[0].parse::<usize>()?;
                let end = range[1].parse::<usize>()?;

                all_ranges.push((start, end));

                continue 'lines;
            }

            if line == "" {
                continue 'lines;
            }

            let id = line.parse::<usize>()?;
            for (_, (start, end)) in all_ranges.iter().enumerate() {
                if id >= *start && id <= *end {
                    fresh_id += 1;
                    continue 'lines;
                }
            }
        }

        Ok(fresh_id)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut fresh_ids: Vec<(usize, usize)> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if line == "" {
                break;
            }

            if line.contains('-') {
                let range: Vec<&str> = line.split('-').collect();
                let start = range[0].parse::<usize>()?;
                let end = range[1].parse::<usize>()?;

                fresh_ids.push((start, end));
            }
        }
        fresh_ids.sort_by(|a, b| a.0.cmp(&b.0));

        let mut compact_ranges: Vec<(usize, usize)> = Vec::new();
        for (start, end) in fresh_ids.iter() {
            match compact_ranges.last_mut() {
                Some(range) => {
                    if range.1 >= *start && range.1 <= *end {
                        range.1 = *end;
                    } else if range.1 < *start {
                        compact_ranges.push((*start, *end));
                    }
                }

                None => {
                    compact_ranges.push((*start, *end));
                }
            }
        }

        let mut fresh_id = 0;
        compact_ranges.iter().for_each(|(start, end)| {
            fresh_id += (end - start + 1);
        });
        Ok(fresh_id)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}
