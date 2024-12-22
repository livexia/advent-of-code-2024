use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

use itertools::Itertools;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<isize>> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect())
}

fn mix(secret: isize, value: isize) -> isize {
    secret ^ value
}

fn prune(secret: isize) -> isize {
    secret % 16777216
}

fn next_secret(secret: isize) -> isize {
    let s1 = prune(mix(secret, secret * 64));
    let s2 = prune(mix(s1, s1 / 32));
    prune(mix(s2, s2 * 2048))
}

fn part1(secrets: &[isize]) -> Result<isize> {
    let _start = Instant::now();

    let mut result = 0;
    for &secret in secrets {
        let mut secret = secret;
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        result += secret;
    }

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn get_price_change(mut secret: isize) -> (Vec<i8>, Vec<i8>) {
    let mut price = Vec::with_capacity(2000);
    let mut change = Vec::with_capacity(2000);
    let mut last = (secret % 10) as i8;
    for _ in 0..2000 {
        secret = next_secret(secret);
        let cur = (secret % 10) as i8;
        price.push(cur);
        change.push(cur - last);
        last = cur;
    }
    (price, change)
}

fn part2(secrets: &[isize]) -> Result<isize> {
    let _start = Instant::now();

    let mut seq_price = HashMap::with_capacity(2000 * 10);
    for (price, change) in secrets.iter().map(|&s| get_price_change(s)) {
        let mut seen = HashSet::with_capacity(2000);
        for (i, (&s0, &s1, &s2, &s3)) in change.iter().tuple_windows().enumerate() {
            let seq = (s0, s1, s2, s3);
            if seen.insert(seq) {
                *seq_price.entry(seq).or_default() += price[i + 3] as isize;
            }
        }
    }

    let &result = seq_price.values().max().unwrap();

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let secrets = parse_input(input)?;
    part1(&secrets)?;
    part2(&secrets)?;
    Ok(())
}

#[test]
fn example_input1() -> Result<()> {
    let input = "1
10
100
2024";
    let secrets = parse_input(input)?;
    assert_eq!(part1(&secrets)?, 37327623);
    Ok(())
}

#[test]
fn example_input2() -> Result<()> {
    let input = "1
2
3
2024";
    let secrets = parse_input(input)?;
    assert_eq!(part2(&secrets)?, 23);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let secrets = parse_input(input)?;
    assert_eq!(part1(&secrets)?, 15335183969);
    assert_eq!(part2(&secrets)?, 1696);
    Ok(())
}
