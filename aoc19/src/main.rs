use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<String>, Vec<String>)> {
    let mut patterns = vec![];
    let mut designs = vec![];
    for (i, l) in input.as_ref().trim().lines().enumerate() {
        if l.is_empty() {
            continue;
        }
        if i == 0 {
            patterns = l.trim().split(", ").map(|p| p.trim().to_string()).collect();
        } else {
            designs.push(l.trim().to_string());
        }
    }
    Ok((patterns, designs))
}

fn is_possible(patterns: &Vec<String>, design: &str) -> bool {
    design.is_empty()
        || patterns
            .iter()
            .filter(|&pat| design.starts_with(pat))
            .any(|pat| is_possible(patterns, &design[pat.len()..]))
}

fn part1(patterns: &Vec<String>, designs: &[String]) -> Result<usize> {
    let _start = Instant::now();

    let result = designs
        .iter()
        .filter(|design| is_possible(patterns, design))
        .count();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn possible_count(
    patterns: &Vec<String>,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(r) = cache.get(design) {
        return *r;
    }
    let mut r = 0;
    if design.is_empty() {
        r += 1;
    }
    r += patterns
        .iter()
        .filter(|&pat| design.starts_with(pat))
        .map(|pat| possible_count(patterns, &design[pat.len()..], cache))
        .sum::<usize>();
    cache.insert(design.to_owned(), r);
    r
}

fn part2(patterns: &Vec<String>, designs: &[String]) -> Result<usize> {
    let _start = Instant::now();

    let mut cache = HashMap::new();
    let result = designs
        .iter()
        .map(|design| possible_count(patterns, design, &mut cache))
        .sum();

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (pattterns, designs) = parse_input(input)?;
    part1(&pattterns, &designs)?;
    part2(&pattterns, &designs)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    let (pattterns, designs) = parse_input(input)?;
    assert_eq!(part1(&pattterns, &designs)?, 6);
    assert_eq!(part2(&pattterns, &designs)?, 16);
    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (pattterns, designs) = parse_input(input)?;
    assert_eq!(part1(&pattterns, &designs)?, 236);
    assert_eq!(part2(&pattterns, &designs)?, 643685981770598);
    Ok(())
}
