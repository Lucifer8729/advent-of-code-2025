use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut split_count = 0;

        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut beams: HashSet<usize> = HashSet::new();
        let mut new_beams: HashSet<usize> = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains('S') {
                beams.insert(line.find('S').unwrap());
            }
            grid.push(line.chars().collect());
        }

        for i in 1..grid.len() {
            for j in 0..grid[i].len() {
                // if grid[i][j] != '^' {
                //     continue;
                // }
                if beams.contains(&j) {
                    if grid[i][j] == '^' {
                        grid[i][j - 1] = '|';
                        grid[i][j + 1] = '|';

                        new_beams.insert(j - 1);
                        new_beams.insert(j + 1);

                        split_count += 1;
                        continue;
                    }
                    grid[i][j] = '|';
                    new_beams.insert(j);
                }
            }
            beams = new_beams.clone();
            new_beams.clear();

            // for row in &grid {
            //     for col in row {
            //         print!("{}", col);
            //     }
            //     println!();
            // }
            // println!();
        }

        Ok(split_count)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start = 0;
        for line in reader.lines() {
            let line = line?;
            if line.contains('S') {
                start = line.find('S').unwrap();
            }
            grid.push(line.chars().collect());
        }

        let mut split_count = 0;
        inorder(&mut grid, 1, start, &mut split_count);

        // Reference: https://github.com/AhmedYassineMaalej/AoC-2025/blob/main/src/problems/day7.rs
        // let input = include_str!("../../input/07.txt");
        // let split_count: usize = new_part2(input);
        // Ok(split_count)

        Ok(split_count)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn inorder(grid: &mut Vec<Vec<char>>, row: usize, col: usize, count: &mut usize) {
    if row == grid.len() {
        *count += 1;
        println!("{}", count);
        // for r in grid {
        //     for c in r {
        //         print!("{}", c);
        //     }
        //     println!();
        // }
        // println!();
        return;
    }

    if grid[row][col] == '.' {
        grid[row][col] = '|';
        inorder(grid, row + 1, col, count);
        grid[row][col] = '.';
        return;
    }

    if grid[row][col] == '^' {
        grid[row][col - 1] = '|';
        inorder(grid, row + 1, col - 1, count);
        grid[row][col - 1] = '.';

        grid[row][col + 1] = '|';
        inorder(grid, row + 1, col + 1, count);
        grid[row][col + 1] = '.';
    }
}

// pub fn new_part2(input: &str) -> usize {
//     let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
//     let rows = grid.len();
//     let cols = grid[0].len();
//
//     let start = grid[0].iter().position(|&c| c == b'S').unwrap();
//
//     let mut splits = vec![vec![false; cols]; rows];
//
//     for (row_idx, row) in grid.iter().enumerate() {
//         for (col_idx, cell) in row.iter().enumerate() {
//             if cell == &b'^' {
//                 splits[row_idx][col_idx] = true;
//             }
//         }
//     }
//
//     let mut visited = vec![vec![0usize; cols]; rows];
//
//     count_timelines(0, start, rows, &splits, &mut visited)
// }
//
// fn count_timelines(
//     row: usize,
//     col: usize,
//     row_count: usize,
//     splits: &Vec<Vec<bool>>,
//     visited: &mut Vec<Vec<usize>>,
// ) -> usize {
//     // skip visited cells
//     if visited[row][col] != 0 {
//         return visited[row][col];
//     }
//
//     if row == row_count - 1 {
//         // mark cell as visited
//         visited[row][col] = 1;
//         return 1;
//     }
//
//     // propagate down if not at splitter
//     if !splits[row][col] {
//         // mark cell as visited
//         let timelines = count_timelines(row + 1, col, row_count, splits, visited);
//         visited[row][col] = timelines;
//         return timelines;
//     }
//
//     // split
//     let timelines = count_timelines(row, col + 1, row_count, splits, visited)
//         + count_timelines(row, col - 1, row_count, splits, visited);
//     visited[row][col] = timelines;
//     timelines
// }
