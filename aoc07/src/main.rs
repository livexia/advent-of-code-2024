use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct Equation {
    value: usize,
    operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((left, right)) = s.trim().split_once(":") {
            let value = left.trim().parse()?;
            let operands: Vec<usize> = right
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Ok(Self { value, operands })
        } else {
            err!("unable to parse input: {s:?}")
        }
    }
}

impl Equation {
    fn test_operators_part1(&self) -> bool {
        fn dfs(current: usize, target: usize, operands: &[usize]) -> bool {
            (current == target && operands.is_empty()) || {
                !operands.is_empty()
                    && current <= target
                    && (dfs(current + operands[0], target, &operands[1..])
                        || dfs(current * operands[0], target, &operands[1..]))
            }
        }

        dfs(0, self.value, &self.operands)
    }
    fn test_operators_part2(&self) -> bool {
        fn dfs(current: usize, target: usize, operands: &[usize]) -> bool {
            (current == target && operands.is_empty()) || {
                !operands.is_empty()
                    && current <= target
                    && (dfs(current + operands[0], target, &operands[1..])
                        || dfs(current * operands[0], target, &operands[1..])
                        || dfs(concat(current, operands[0]), target, &operands[1..]))
            }
        }

        fn concat(a: usize, b: usize) -> usize {
            a * 10usize.pow(b.checked_ilog10().unwrap_or(0) + 1) + b
        }

        dfs(0, self.value, &self.operands)
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Equation>> {
    input.as_ref().trim().lines().map(|l| l.parse()).collect()
}

fn part1(equations: &[Equation]) -> Result<usize> {
    let _start = Instant::now();

    let result = equations
        .iter()
        .filter(|e| e.test_operators_part1())
        .map(|e| e.value)
        .sum();
    println!("part1: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(equations: &[Equation]) -> Result<usize> {
    let _start = Instant::now();

    let result = equations
        .iter()
        .filter(|e| e.test_operators_part2())
        .map(|e| e.value)
        .sum();
    println!("part2: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let equations = parse_input(input)?;
    part1(&equations)?;
    part2(&equations)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    let equations = parse_input(input)?;
    assert_eq!(part1(&equations)?, 3749);
    assert_eq!(part2(&equations)?, 11387);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let equations = parse_input(input)?;
    assert_eq!(part1(&equations)?, 3119088655389);
    assert_eq!(part2(&equations)?, 264184041398847);
    Ok(())
}
