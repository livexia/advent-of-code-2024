use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(.+:).*X[+|=](\d*), Y[=|+](\d*)").unwrap();
}

struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for Machine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut result = vec![];
        for (_, [line, x, y]) in RE.captures_iter(s).map(|c| c.extract()) {
            result.push((line, x.parse::<i64>()?, y.parse::<i64>()?));
        }
        if result.len() != 3 {
            err!("unable to parse: {:?}", s)
        } else {
            Ok(Self {
                button_a: (result[0].1, result[0].2),
                button_b: (result[1].1, result[1].2),
                prize: (result[2].1, result[2].2),
            })
        }
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Machine>> {
    input
        .as_ref()
        .split("\n\n")
        .map(|s| s.parse::<Machine>())
        .collect()
}

impl Machine {
    fn min_cost(&self) -> i64 {
        dfs(
            self.prize,
            self.button_a,
            self.button_b,
            &mut HashMap::new(),
        )
    }
}

fn dfs(
    prize: (i64, i64),
    a: (i64, i64),
    b: (i64, i64),
    cache: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    // m(p) = min{m(p - a) + 3, m(p - b) + 1}
    // cache m(p)

    if let Some(s) = cache.get(&prize) {
        return *s;
    }
    if prize.0 == 0 && prize.1 == 0 {
        return 0;
    }
    let mut r = i64::MAX;
    if prize.0 >= a.0 && prize.1 >= a.1 {
        let c = dfs((prize.0 - a.0, prize.1 - a.1), a, b, cache);
        if c != i64::MAX {
            r = r.min(3 + c)
        }
    }
    if prize.0 >= b.0 && prize.1 >= b.1 {
        let c = dfs((prize.0 - b.0, prize.1 - b.1), a, b, cache);
        if c != i64::MAX {
            r = r.min(1 + c)
        }
    }
    cache.insert(prize, r);
    r
}

fn part1(machines: &[Machine]) -> Result<i64> {
    let _start = Instant::now();

    let result = machines
        .iter()
        .map(|m| m.min_cost())
        .filter(|&c| c != i64::MAX)
        .sum();

    println!("part1 with dp: {result}");

    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part1_with_math(machines: &[Machine]) -> Result<i64> {
    let _start = Instant::now();

    let result = machines
        .iter()
        .filter_map(|m| solve(m.prize, m.button_a, m.button_b))
        .map(|(a, b)| a * 3 + b)
        .sum();

    println!("part1 with math: {result}");

    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn solve(p: (i64, i64), a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    // p.0 = a.0 * x + b.0 * y
    // p.1 = a.1 * x + b.1 * y
    // p.0 * b.1 = a.0 * b.1 * x + b.0 * b.1 * y
    // p.1 * b.0 = a.1 * b.0 * x + b.0 * b.1 * y
    // p.0 * b.1 - p.1 * b. 0 = (a.0 * b.1 - a.1 * b.0)x
    // x = (p.0 * b.1 - p.1 * b.0) / (a.0 * b.1 - a.1 * b.0)
    let x = (p.0 * b.1 - p.1 * b.0, a.0 * b.1 - a.1 * b.0);
    let y = (p.0 * a.1 - p.1 * a.0, b.0 * a.1 - b.1 * a.0);
    assert_ne!(x.1, 0);
    assert_ne!(y.1, 0);
    if x.0 % x.1 == 0 && y.0 % y.1 == 0 {
        Some((x.0 / x.1, y.0 / y.1))
    } else {
        None
    }
}

fn part2(machines: &[Machine]) -> Result<i64> {
    let _start = Instant::now();

    let offset = 10000000000000;

    let result: i64 = machines
        .iter()
        .filter_map(|m| {
            solve(
                (m.prize.0 + offset, m.prize.1 + offset),
                m.button_a,
                m.button_b,
            )
        })
        .map(|(a, b)| a * 3 + b)
        .sum();
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let machines = parse_input(input)?;

    part1(&machines)?;
    part1_with_math(&machines)?;
    part2(&machines)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    let machines = parse_input(input)?;

    assert_eq!(part1(&machines)?, 480);
    assert_eq!(part1_with_math(&machines)?, 480);
    assert_eq!(part2(&machines)?, 875318608908);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let machines = parse_input(input)?;

    assert_eq!(part1(&machines)?, 31623);
    assert_eq!(part1_with_math(&machines)?, 31623);
    assert_eq!(part2(&machines)?, 93209116744825);
    assert_eq!(2, 2);
    Ok(())
}
