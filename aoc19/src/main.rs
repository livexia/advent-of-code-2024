use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Towel = Vec<u8>;
fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<Towel>, Vec<Towel>)> {
    let mut patterns = vec![];
    let mut designs = vec![];
    for (i, l) in input.as_ref().trim().lines().enumerate() {
        if l.is_empty() {
            continue;
        }
        if i == 0 {
            patterns = l
                .trim()
                .split(", ")
                .map(|p| p.trim().bytes().collect())
                .collect();
        } else {
            designs.push(l.trim().bytes().collect());
        }
    }
    Ok((patterns, designs))
}

fn is_possible(patterns: &[Towel], design: &Towel) -> bool {
    if design.is_empty() || patterns.contains(design) {
        true
    } else {
        for pattern in patterns {
            let l = pattern.len();
            if design.len() > l
                && pattern == &design[..l]
                && is_possible(patterns, &design[l..].to_vec())
            {
                return true;
            }
        }
        false
    }
}

fn part1(patterns: &[Towel], designs: &[Towel]) -> Result<usize> {
    let _start = Instant::now();

    let result = designs
        .iter()
        .filter(|design| is_possible(patterns, design))
        .count();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn possible_count(patterns: &[Towel], design: &Towel, cache: &mut HashMap<Towel, usize>) -> usize {
    if let Some(r) = cache.get(design) {
        return *r;
    }
    let mut r = 0;
    if design.is_empty() || patterns.contains(design) {
        r += 1;
    }
    for pattern in patterns {
        let l = pattern.len();
        if design.len() > l && pattern == &design[..l] {
            let nr = possible_count(patterns, &design[l..].to_vec(), cache);
            r += nr;
        }
    }
    cache.insert(design.to_vec(), r);
    r
}

fn part2(patterns: &[Towel], designs: &[Towel]) -> Result<usize> {
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
    assert_eq!(2, 2);
    Ok(())
}
