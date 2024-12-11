use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<usize>> {
    Ok(input
        .as_ref()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect())
}

fn transform(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![1]
    } else if (n.ilog10() + 1) % 2 == 0 {
        let l = n.ilog10() + 1;
        vec![n / (10usize.pow(l / 2)), n % (10usize.pow(l / 2))]
    } else {
        vec![n * 2024]
    }
}

fn transform_stones(stones: &[usize], times: usize) -> usize {
    let mut stones_count: HashMap<usize, usize> = stones.iter().map(|&s| (s, 1)).collect();
    let mut transform_cached: HashMap<usize, Vec<usize>> = HashMap::new();
    for _i in 0..times {
        let mut tmp = HashMap::new();
        for (&stone, &count) in &stones_count {
            let e = transform_cached.entry(stone).or_insert(transform(stone));
            for s in e {
                *tmp.entry(*s).or_default() += count;
            }
        }
        println!("{_i} 55360: {:?}", stones_count.get(&55360));
        stones_count = tmp;
    }
    stones_count.values().sum()
}

fn part1(stones: &[usize]) -> Result<usize> {
    let _start = Instant::now();

    let result = transform_stones(stones, 25);
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(stones: &[usize]) -> Result<usize> {
    let _start = Instant::now();

    let result = transform_stones(stones, 75);
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let stones = parse_input(input)?;

    part1(&stones)?;
    part2(&stones)?;
    Ok(())
}

#[test]
fn test_transform() -> Result<()> {
    assert_eq!(transform(0), vec![1]);
    assert_eq!(transform(1000), vec![10, 0]);
    assert_eq!(transform(999), vec![2021976]);
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "125 17";
    let stones = parse_input(input)?;
    assert_eq!(part1(&stones)?, 55312);
    assert_eq!(part2(&stones)?, 65601038650482);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let stones = parse_input(input)?;
    assert_eq!(part1(&stones)?, 203228);
    assert_eq!(part2(&stones)?, 240884656550923);
    Ok(())
}
