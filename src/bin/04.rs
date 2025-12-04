use adv_code_2025::start_day;
use anyhow::Result;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut rolls = 0;

        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut row: Vec<char> = Vec::new();
            line.chars().for_each(|c| {
                row.push(c);
            });
            grid.push(row);
        }

        for (i, row) in grid.iter().enumerate() {
            for (j, _col) in row.iter().enumerate() {
                if can_access(&grid, i, j) {
                    rolls += 1;
                }
            }
        }

        Ok(rolls)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part1(input_file)?;
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut rolls = 0;

        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut row: Vec<char> = Vec::new();
            line.chars().for_each(|c| {
                row.push(c);
            });
            grid.push(row);
        }

        while can_remove(&grid) {
            let mut cells: Vec<(usize, usize)> = Vec::new();
            for (i, row) in grid.iter().enumerate() {
                for (j, _col) in row.iter().enumerate() {
                    if can_access(&grid, i, j) {
                        rolls += 1;
                        cells.push((i, j));
                    }
                }
            }

            for (i, j) in cells {
                grid[i][j] = 'x';
            }
        }

        Ok(rolls)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = part2(input_file)?;
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn can_remove(grid: &Vec<Vec<char>>) -> bool {
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if can_access(&grid, i, j) {
                return true;
            }
        }
    }
    false
}

fn can_access(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] == '.' || grid[row][col] == 'x' {
        return false;
    }
    let directions: Vec<(i32, i32)> = Vec::from([
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ]);

    let mut neighbours = 0;
    for (dx, dy) in directions {
        let x = row as i32 + dx;
        let y = col as i32 + dy;

        if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32 {
            continue;
        }

        if grid[x as usize][y as usize] == '@' {
            neighbours += 1;
            if neighbours > 3 {
                return false;
            }
        }
    }

    true
}
