use regex::Regex;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Instruction {
    Mul(isize, isize),
    Do,
    Donot,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        if let Some(s) = s.strip_prefix("mul(") {
            if let Some(s) = s.strip_suffix(")") {
                if let Some((l, r)) = s.split_once(',') {
                    let l: isize = l.trim().parse()?;
                    let r: isize = r.trim().parse()?;
                    return Ok(Instruction::Mul(l, r));
                }
            }
        } else if s == "do()" {
            return Ok(Instruction::Do);
        } else if s == "don't()" {
            return Ok(Instruction::Donot);
        }
        err!("Unable to parse for {:?}", s)
    }
}
impl Instruction {
    fn run(&self) -> isize {
        match self {
            Instruction::Mul(l, r) => l * r,
            Instruction::Do => 0,
            Instruction::Donot => 0,
        }
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Instruction>> {
    let mut instrs = vec![];
    let re = Regex::new(r"(mul\(\d+\s*,\d+\))|(do\(\))|(don't\(\))").unwrap();
    for (_, [instr_raw]) in re.captures_iter(input.as_ref()).map(|c| c.extract()) {
        instrs.push(instr_raw.parse()?);
    }
    Ok(instrs)
}

fn part1(instrs: &[Instruction]) -> Result<isize> {
    let _start = Instant::now();

    let result = instrs.iter().map(|i| i.run()).sum();

    println!("part1: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(instrs: &[Instruction]) -> Result<isize> {
    let _start = Instant::now();

    let mut result = 0;
    let mut enabled = true;
    for instr in instrs {
        match instr {
            Instruction::Mul(_, _) => result += enabled as isize * instr.run(),
            Instruction::Do => enabled = true,
            Instruction::Donot => enabled = false,
        }
    }

    println!("part2: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instrs = parse_input(input)?;

    part1(&instrs)?;
    part2(&instrs)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let instrs = parse_input(input).unwrap();

    assert_eq!(part1(&instrs).unwrap(), 161);
    assert_eq!(part2(&instrs).unwrap(), 48);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let instrs = parse_input(input).unwrap();

    assert_eq!(part1(&instrs).unwrap(), 178886550);
    assert_eq!(part2(&instrs).unwrap(), 87163705);
}
