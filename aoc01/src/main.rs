use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn part1(list: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let mut left: Vec<_> = list.iter().map(|v| v[0]).collect();
    let mut right: Vec<_> = list.iter().map(|v| v[1]).collect();

    left.sort();
    right.sort();

    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |s, (a, b)| s + a.abs_diff(*b));
    println!("part1: {result}");

    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(list: &[Vec<usize>]) -> Result<usize> {
    let _start = Instant::now();

    let mut count: HashMap<usize, usize> = HashMap::new();
    for v in list {
        *count.entry(v[1]).or_default() += 1;
    }

    let result = list
        .iter()
        .fold(0, |s, v| s + v[0] * count.get(&v[0]).unwrap_or(&0));

    println!("part2: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let list = parse_input(&input);

    part1(&list)?;
    part2(&list)?;
    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn example_input() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let list = parse_input(input);
    assert_eq!(part1(&list).unwrap(), 11);
    assert_eq!(part2(&list).unwrap(), 31);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let list = parse_input(&input);
    assert_eq!(part1(&list).unwrap(), 2166959);
    assert_eq!(part2(&list).unwrap(), 23741109);
}
