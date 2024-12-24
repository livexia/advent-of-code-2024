use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Gate = [String; 4];

fn parse_input<T: AsRef<str>>(input: T) -> Result<(HashMap<String, usize>, Vec<Gate>)> {
    if let Some((l, r)) = input.as_ref().trim().split_once("\n\n") {
        let wires: HashMap<_, _> = l
            .trim()
            .lines()
            .filter_map(|l| l.split_once(": "))
            .map(|(name, value)| {
                (
                    name.trim().to_string(),
                    value.trim().parse::<usize>().unwrap(),
                )
            })
            .collect();
        let gates: Vec<_> = r
            .trim()
            .lines()
            .filter_map(|l| l.split_once(" -> "))
            .map(|(lhs, rhs)| {
                let lhs: Vec<_> = lhs.split_whitespace().collect();
                let rhs = rhs.trim().to_string();
                [
                    lhs[0].to_string(),
                    lhs[1].to_string(),
                    lhs[2].to_string(),
                    rhs,
                ]
            })
            .collect();
        return Ok((wires, gates));
    }
    err!("unable to parse input")
}

fn eval(o1: usize, op: &str, o2: usize) -> usize {
    match op {
        "AND" => o1 & o2,
        "OR" => o1 | o2,
        "XOR" => o1 ^ o2,
        _ => unreachable!("unknow gate: {:?}", op),
    }
}

fn run_circuit(wires: &HashMap<String, usize>, gates: &[Gate]) -> usize {
    let mut wires = wires.to_owned();
    let mut queue = VecDeque::new();
    queue.extend(gates.iter().cloned());

    while let Some([o1, op, o2, rhs]) = queue.pop_front() {
        if let Some(&o1) = wires.get(&o1) {
            if let Some(&o2) = wires.get(&o2) {
                wires.insert(rhs.to_string(), eval(o1, &op, o2));
                continue;
            }
        }
        queue.push_back([o1, op, o2, rhs]);
    }

    dec(&wires, "z")
}

fn part1(wires: &HashMap<String, usize>, gates: &[Gate]) -> Result<usize> {
    let _start = Instant::now();

    let result = run_circuit(wires, gates);

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn dec(wires: &HashMap<String, usize>, prefix: &str) -> usize {
    let mut num: Vec<_> = wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect();
    num.sort();
    num.iter()
        .rev()
        .map(|(_, v)| v)
        .fold(0, |s, &&b| s << 1 | b)
}

#[derive(Debug, Clone)]
enum Formula {
    Value(String),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Xor(Box<Formula>, Box<Formula>),
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::And(l0, l1), Self::And(r0, r1))
            | (Self::Or(l0, l1), Self::Or(r0, r1))
            | (Self::Xor(l0, l1), Self::Xor(r0, r1)) => {
                (l0 == r0 && l1 == r1) || (l0 == r1 && l1 == r0)
            }
            _ => false,
        }
    }
}

impl Formula {
    #[allow(dead_code)]
    fn eval(&self, wires: &HashMap<String, usize>) -> usize {
        match self {
            Formula::Value(w) => *wires.get(w).unwrap(),
            Formula::And(formula, formula1) => formula.eval(wires) & formula1.eval(wires),
            Formula::Or(formula, formula1) => formula.eval(wires) | formula1.eval(wires),
            Formula::Xor(formula, formula1) => formula.eval(wires) ^ formula1.eval(wires),
        }
    }
}

fn formula_from_input(rhs: &str, equations: &HashMap<String, (String, String, String)>) -> Formula {
    use Formula::*;
    if let Some(ops) = equations.get(rhs) {
        let o1 = Box::new(formula_from_input(&ops.0, equations));
        let o2 = Box::new(formula_from_input(&ops.2, equations));
        match ops.1.as_str() {
            "AND" => And(o1, o2),
            "XOR" => Xor(o1, o2),
            "OR" => Or(o1, o2),
            _ => unreachable!("unknow formula: {ops:?}"),
        }
    } else {
        Value(rhs.to_string())
    }
}

fn addition_formula(rhs: usize) -> (Formula, Formula) {
    // carry_in, a, b
    // sum = (carry in) xor (a xor b)
    // carry_out = (carry in) and (a xor b) or (a and b)
    // z0 = x0 xor y0
    // carry = x0 and y0
    // z1 = (x0 and y0) xor (x1 xor y1)

    use Formula::*;
    let x = Box::new(Value(format!("x{rhs:02}")));
    let y = Box::new(Value(format!("y{rhs:02}")));
    if rhs == 0 {
        (Xor(x.clone(), y.clone()), And(x.clone(), y.clone()))
    } else {
        let (_, c) = addition_formula(rhs - 1);
        let xor = Box::new(Xor(x.clone(), y.clone()));
        let and = Box::new(And(x.clone(), y.clone()));
        (
            Xor(Box::new(c.clone()), xor.clone()),
            Or(and, Box::new(And(xor, Box::new(c)))),
        )
    }
}

fn part2(wires: &HashMap<String, usize>, gates: &[[String; 4]]) -> Result<String> {
    let _start = Instant::now();

    let x = dec(wires, "x");
    let y = dec(wires, "y");
    let expected_z = x + y;
    let wrong_z = run_circuit(wires, gates);

    println!("{x} + {y} = {expected_z}");
    println!("correct: {expected_z:0b}");
    println!("wrong  : {wrong_z:0b}");

    let equations: HashMap<_, _> = gates
        .iter()
        .cloned()
        .map(|[o1, op, o2, rhs]| (rhs, (o1, op, o2)))
        .collect();

    for i in 0..wires.len() / 2 {
        let rhs = format!("z{i:02}");
        let input_formula = formula_from_input(&rhs, &equations);
        let true_formula = addition_formula(i).0;
        if input_formula != true_formula {
            println!("different at bit: {i}, try to fix it by hand, then re-run");
            // println!("{:?}", input_formula);
            // println!("{:?}", true_formula);
            break;
        }
    }

    assert_eq!(expected_z, run_circuit(wires, gates));

    let result = String::new();
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (wires, gates) = parse_input(input)?;
    part1(&wires, &gates)?;
    part2(&wires, &gates)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    let (wires, gates) = parse_input(input)?;
    assert_eq!(part1(&wires, &gates)?, 2024);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (wires, gates) = parse_input(input)?;
    assert_eq!(part1(&wires, &gates)?, 52038112429798);
    Ok(())
}
