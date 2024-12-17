use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Integer = isize;

#[derive(Debug, Clone)]
struct Computer {
    program: Vec<Integer>,
    pc: usize,
    registers: [Integer; 3],
}

impl FromStr for Computer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut registers = [0; 3];
        let mut program = vec![];
        for l in s.trim().lines().filter(|l| !l.trim().is_empty()) {
            if let Some(r) = l.strip_prefix("Register A: ") {
                registers[0] = r.trim().parse().unwrap();
            } else if let Some(r) = l.trim().strip_prefix("Register B: ") {
                registers[1] = r.trim().parse().unwrap();
            } else if let Some(r) = l.trim().strip_prefix("Register C: ") {
                registers[2] = r.trim().parse().unwrap();
            } else if let Some(p) = l.trim().strip_prefix("Program: ") {
                program = p.split(",").map(|n| n.trim().parse().unwrap()).collect();
            }
        }
        Ok(Self {
            program,
            pc: 0,
            registers,
        })
    }
}

impl Computer {
    fn combo_operand(&self) -> Integer {
        let operand = self.program[self.pc + 1];
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => unreachable!("reserved operand"),
            _ => unreachable!("unknown operand: {operand:?}"),
        }
    }
    fn cycle(&mut self) -> Option<Integer> {
        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        let mut jumped = false;
        match opcode {
            0 => self.registers[0] /= 2isize.pow(self.combo_operand() as u32),
            1 => self.registers[1] ^= operand,
            2 => self.registers[1] = self.combo_operand() & 0b111,
            3 => {
                if self.registers[0] != 0 {
                    jumped = true;
                    self.pc = operand as usize;
                }
            }
            4 => self.registers[1] ^= self.registers[2],
            5 => {
                let operand = self.combo_operand();
                self.pc += 2;
                return Some(operand & 0b111);
            }
            6 => self.registers[1] = self.registers[0] / 2isize.pow(self.combo_operand() as u32),
            7 => self.registers[2] = self.registers[0] / 2isize.pow(self.combo_operand() as u32),
            _ => unreachable!("Unknown opcode: {:?}", opcode),
        }

        if !jumped {
            self.pc += 2;
        }
        None
    }

    fn is_halt(&self) -> bool {
        self.pc >= self.program.len()
    }
}

fn part1(computer: &Computer) -> Result<String> {
    let _start = Instant::now();

    let mut computer = computer.clone();
    let mut output = vec![];
    while !computer.is_halt() {
        if let Some(o) = computer.cycle() {
            output.push(o);
        }
    }

    let result = output
        .iter()
        .map(|n| format!("{n}"))
        .collect::<Vec<_>>()
        .join(",");
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2_with_simplified_program(computer: &Computer) -> Result<Integer> {
    let _start = Instant::now();

    assert_eq!(computer.program, vec![
        2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 0, 5, 5, 3, 0
    ]);

    // 2, 4 => B = A & 0b111
    // 1, 1 => B = B ^ 1
    // 7, 5 => C = A / 2^B
    // 0, 3 => A = A / 2^3
    // 1, 4 => B = B ^ 4
    // 4, 0 => B = B ^ C
    // 5, 5 => output: B & 0b111
    // 3, 0 => if A != 0 jump 0

    fn dfs(a: isize, i: usize, target: &[isize]) -> Option<isize> {
        for current in 0..8 {
            // let o = ((((a & 0b111) ^ 1) ^ 4) ^ (a / (2isize.pow(a as u32 & 0b111 ^ 1)))) & 0b111;
            // let o = (a & 0b111 ^ 5) ^ ((a >> (a as u32 & 0b111 ^ 1)) & 0b111);
            let a = (a << 3) + current;
            let o = a & 0b111 ^ 0b101 ^ (a >> (a & 0b111 ^ 1)) & 0b111;
            if o == target[i] {
                if i == 0 {
                    return Some(a);
                }
                if let Some(a) = dfs(a, i - 1, target) {
                    return Some(a);
                }
            }
        }
        None
    }

    let result = dfs(0, computer.program.len() - 1, &computer.program).unwrap();

    println!("part2 with simplfied program(only work for my input): {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2_with_sim(computer: &Computer) -> Result<Integer> {
    let _start = Instant::now();

    fn dfs_with_sim(a: isize, i: usize, computer: &Computer) -> Option<isize> {
        for current in 0..8 {
            let mut computer_alt = computer.clone();
            let a = (a << 3) + current;
            computer_alt.registers[0] = a;
            while !computer_alt.is_halt() {
                if let Some(o) = computer_alt.cycle() {
                    if o == computer_alt.program[i] {
                        if i == 0 {
                            return Some(a);
                        }
                        if let Some(a) = dfs_with_sim(a, i - 1, computer) {
                            return Some(a);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        None
    }
    let result = dfs_with_sim(0, computer.program.len() - 1, computer).unwrap();

    println!("part2 with run computer: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let computer = input.parse()?;
    part1(&computer)?;
    part2_with_sim(&computer)?;
    part2_with_simplified_program(&computer)?;
    Ok(())
}

#[test]
fn test_instr() -> Result<()> {
    let mut computer = Computer {
        program: vec![],
        pc: 0,
        registers: [0; 3],
    };
    computer.registers[2] = 9;
    computer.program = vec![2, 6];
    computer.cycle();
    assert_eq!(computer.registers[1], 1);

    let mut computer = Computer {
        program: vec![],
        pc: 0,
        registers: [0; 3],
    };
    computer.registers[0] = 10;
    computer.program = vec![5, 0, 5, 1, 5, 4];
    assert_eq!(part1(&computer)?, "0,1,2");
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let computer = input.parse()?;
    assert_eq!(part1(&computer)?, "4,6,3,5,6,3,5,2,1,0");
    Ok(())
}

#[test]
fn example_input2() -> Result<()> {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    let computer = input.parse()?;
    assert_eq!(part2_with_sim(&computer)?, 117440);
    Ok(())
}
#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let computer = input.parse()?;
    assert_eq!(part1(&computer)?, "7,1,2,3,2,6,7,2,5");
    assert_eq!(part2_with_simplified_program(&computer)?, 202356708354602);
    assert_eq!(part2_with_sim(&computer)?, 202356708354602);
    assert_eq!(2, 2);
    Ok(())
}
