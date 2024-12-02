use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<usize>> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(row: &[usize]) -> bool {
    let flag = row[0].cmp(&row[1]);
    if flag == std::cmp::Ordering::Equal {
        return false;
    }
    for (i, j) in row.iter().zip(row.iter().skip(1)) {
        if i.abs_diff(*j) <= 3 && i.cmp(j) == flag {
            continue;
        } else {
            return false;
        }
    }
    true
}

fn part1(data: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    for row in data {
        if is_safe(row) {
            result += 1;
        }
    }
    println!("part1: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(data: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    for row in data {
        if is_safe(row) {
            result += 1;
            continue;
        }
        for index in 0..row.len() {
            let mut row = row.clone();
            row.remove(index);
            if is_safe(&row) {
                result += 1;
                break;
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

    let data = parse_input(&input);
    part1(&data)?;
    part2(&data)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let data = parse_input(input);
    assert_eq!(part1(&data).unwrap(), 2);
    assert_eq!(part2(&data).unwrap(), 4);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let data = parse_input(input);
    assert_eq!(part1(&data).unwrap(), 510);
    assert_eq!(part2(&data).unwrap(), 553);
}
