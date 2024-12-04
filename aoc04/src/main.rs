use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Grid = Vec<Vec<char>>;

fn parse_input<T: AsRef<str>>(input: T) -> Grid {
    input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn search_part1(grid: &Grid, i: usize, j: usize) -> usize {
    let pattern = if grid[i][j] == 'X' {
        ['X', 'M', 'A', 'S']
    } else if grid[i][j] == 'S' {
        ['S', 'A', 'M', 'X']
    } else {
        return 0;
    };
    let l = pattern.len();
    let w = grid[0].len();
    let h = grid.len();
    (j + l < w && grid[i][j..j + l] == pattern) as usize
        + (i + l <= h && (0..l).all(|offset| grid[i + offset][j] == pattern[offset])) as usize
        + (i + l <= h
            && j + 1 >= l
            && (0..l).all(|offset| grid[i + offset][j - offset] == pattern[offset]))
            as usize
        + (i + l <= h
            && j + l <= w
            && (0..l).all(|offset| grid[i + offset][j + offset] == pattern[offset]))
            as usize
}

fn part1(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let width = grid[0].len();
    let height = grid.len();

    let mut result = 0;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'X' || grid[i][j] == 'S' {
                result += search_part1(grid, i, j);
            }
        }
    }

    println!("part1: {result}");

    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn search_part2(grid: &Grid, i: usize, j: usize) -> usize {
    // if grid[i][j] != 'A' {
    //     return 0;
    // }
    // if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[0].len() - 1 {
    //     return 0;
    // }
    let w1 = [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
    let w2 = [grid[i - 1][j + 1], grid[i][j], grid[i + 1][j - 1]];
    [w1, w2]
        .iter()
        .all(|w| [['M', 'A', 'S'], ['S', 'A', 'M']].contains(w)) as usize
}

fn part2(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let width = grid[0].len();
    let height = grid.len();

    let mut result = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if grid[i][j] == 'A' {
                result += search_part2(grid, i, j);
            }
        }
    }

    println!("part2: {result}");

    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(input);
    part1(&grid)?;
    part2(&grid)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 18);
    assert_eq!(part2(&grid).unwrap(), 9);
    assert_eq!(1, 1);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 2493);
    assert_eq!(part2(&grid).unwrap(), 1890);
    assert_eq!(2, 2);
}
