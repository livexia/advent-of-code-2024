use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::iter;
use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Vec<char>>> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect())
}

const NUMERIC_KEYPAD: [[char; 3]; 4] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [
    '*', '0', 'A',
]];
const NUMERIC_KEYPAD_A: (usize, usize) = (3, 2);
const NUMERIC_KEYPAD_EMPTY: (usize, usize) = (3, 0);
const DIRECTIONAL_KAYPAD: [[char; 3]; 2] = [['*', '^', 'A'], ['<', 'v', '>']];
const DIRECTIONAL_KAYPAD_A: (usize, usize) = (0, 2);
const DIRECTIONAL_KAYPAD_EMPTY: (usize, usize) = (0, 0);

fn complexity(code: &[char]) -> usize {
    code.iter()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |s, &c| s * 10 + (c as u8 - b'0') as usize)
}

fn is_valid_move(
    key: char,
    coord: (usize, usize),
    keypad: &[[char; 3]],
    empty_key: (usize, usize),
) -> bool {
    let (dx, dy) = match key {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        'A' => return true,
        _ => unreachable!("no key press with {key:?}"),
    };
    let (x, y) = (coord.0 as isize, coord.1 as isize);
    !(x + dx < 0
        || y + dy < 0
        || x + dx >= keypad.len() as isize
        || y + dy >= keypad[0].len() as isize
        || (x + dx == empty_key.0 as isize && y + dy == empty_key.1 as isize))
}

fn press(key: char, coords: &mut [(usize, usize)]) -> Result<Option<char>> {
    // first robot: v<A move robot arm with v< and press A to send v to next robot
    // second robot: v move robot arm with v press nothing
    // third robor: there is nothing to do
    // fouth robot: there is nothing to do
    let length = coords.len();
    if (length == 1 && !is_valid_move(key, coords[0], &NUMERIC_KEYPAD, NUMERIC_KEYPAD_EMPTY))
        || (length != 1
            && !is_valid_move(
                key,
                coords[0],
                &DIRECTIONAL_KAYPAD,
                DIRECTIONAL_KAYPAD_EMPTY,
            ))
    {
        return err!("unable to move {:?} with {:?}", coords[0], key);
    }
    let (x, y) = &mut coords[0];
    match key {
        '^' => *x -= 1,
        'v' => *x += 1,
        '>' => *y += 1,
        '<' => *y -= 1,
        'A' => {
            if length == 1 {
                return Ok(Some(NUMERIC_KEYPAD[*x][*y]));
            } else {
                return press(DIRECTIONAL_KAYPAD[*x][*y], &mut coords[1..]);
            };
        }
        _ => unreachable!("no key press with {key:?}"),
    }
    Ok(None)
}

fn shortest_press(code: &[char], coords: Vec<(usize, usize)>) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((coords, 0, 0));
    let mut visited = HashSet::new();
    while let Some((coords, length, code_index)) = queue.pop_front() {
        if code_index == code.len() {
            return Some(length);
        }
        // if visited.insert((coords.clone(), code_index)) {
        for key in ['^', 'v', '>', '<', 'A'] {
            let mut alt_coords = coords.clone();
            match press(key, &mut alt_coords) {
                Ok(r) => {
                    if r == Some(code[code_index]) {
                        if visited.insert((alt_coords.clone(), code_index + 1)) {
                            queue.push_back((alt_coords, length + 1, code_index + 1));
                        }
                    } else if r.is_none() {
                        if visited.insert((alt_coords.clone(), code_index)) {
                            queue.push_back((alt_coords, length + 1, code_index));
                        }
                    } else {
                        continue;
                    }
                }
                Err(_e) => continue,
            }
        }
        // }
    }
    None
}

fn all_coords(d_count: usize, n_count: usize) -> Vec<(usize, usize)> {
    iter::repeat(DIRECTIONAL_KAYPAD_A)
        .take(d_count)
        .chain(iter::repeat(NUMERIC_KEYPAD_A).take(n_count))
        .collect()
}

fn part1(codes: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let result = codes
        .par_iter()
        .map(|code| {
            let coords = all_coords(2, 1);
            shortest_press(code, coords).unwrap() * complexity(code)
        })
        .sum();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(codes: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let result: usize = codes
        .par_iter()
        .map(|code| {
            let coords = all_coords(25, 1);
            shortest_press(code, coords).unwrap() * complexity(code)
        })
        .sum();
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let codes = parse_input(input)?;

    part1(&codes)?;
    part2(&codes)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "029A
980A
179A
456A
379A";
    let codes = parse_input(input)?;
    assert_eq!(part1(&codes)?, 126384);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let codes = parse_input(input)?;
    assert_eq!(part1(&codes)?, 219366);
    assert_eq!(part2(&codes)?, 219366);
    assert_eq!(2, 2);
    Ok(())
}
