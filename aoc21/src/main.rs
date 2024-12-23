use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::iter::repeat;
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

type Coord = (usize, usize);
const NUMERIC_KEYPAD: [[char; 3]; 4] = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [
    '*', '0', 'A',
]];
const NUMERIC_KEYPAD_A: Coord = (3, 2);
const NUMERIC_KEYPAD_EMPTY: Coord = (3, 0);
const DIRECTIONAL_KAYPAD: [[char; 3]; 2] = [['*', '^', 'A'], ['<', 'v', '>']];
const DIRECTIONAL_KAYPAD_A: Coord = (0, 2);
const DIRECTIONAL_KAYPAD_EMPTY: Coord = (0, 0);

fn complexity(code: &[char]) -> usize {
    code.iter()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |s, &c| s * 10 + (c as u8 - b'0') as usize)
}

fn is_valid_move(key: char, coord: Coord, keypad: &[[char; 3]], empty_key: Coord) -> bool {
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

fn press(key: char, coords: &mut [Coord]) -> Result<Option<char>> {
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

fn shortest_press(code: &[char], coords: Vec<Coord>) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((coords, 0, 0));
    let mut visited = HashSet::new();
    while let Some((coords, length, code_index)) = queue.pop_front() {
        if code_index == code.len() {
            return Some(length);
        }
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
    }
    None
}

fn all_coords(d_count: usize, n_count: usize) -> Vec<Coord> {
    repeat(DIRECTIONAL_KAYPAD_A)
        .take(d_count)
        .chain(repeat(NUMERIC_KEYPAD_A).take(n_count))
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

#[allow(dead_code)]
fn dp_keypad(keypad: &[[char; 3]]) -> Vec<Vec<(usize, usize, usize, usize)>> {
    // dp.0 => '^'
    // dp.1 => 'v'
    // dp.2 => '>'
    // dp.3 => '<'
    let (h, w) = (keypad.len(), keypad[0].len());
    let mut dp = vec![vec![(0, 0, 0, 0); h * w]; h * w];
    for i in 0..h {
        for j in 0..w {
            if keypad[i][j] == '*' {
                continue;
            }
            for m in 0..h {
                for n in 0..w {
                    dp[i * 3 + j][m * 3 + n].0 = (m < i) as usize * m.abs_diff(i);
                    dp[i * 3 + j][m * 3 + n].1 = (m > i) as usize * m.abs_diff(i);
                    dp[i * 3 + j][m * 3 + n].2 = (n > j) as usize * n.abs_diff(j);
                    dp[i * 3 + j][m * 3 + n].3 = (n < j) as usize * n.abs_diff(j);
                }
            }
        }
    }
    dp
}

fn keymap(keypad: &[[char; 3]]) -> HashMap<char, Coord> {
    keypad
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (c, (i, j))))
        .collect()
}

fn test_path(mut cur: Coord, path: &[char], keypad: &[[char; 3]], empty_key: Coord) -> bool {
    for &key in path {
        if is_valid_move(key, cur, keypad, empty_key) {
            let (x, y) = &mut cur;
            match key {
                '^' => *x -= 1,
                'v' => *x += 1,
                '>' => *y += 1,
                '<' => *y -= 1,
                _ => unreachable!(),
            }
        } else {
            return false;
        }
    }
    true
}

fn dfs_dp(
    code: &[char],
    deepth: usize,
    max_deepth: usize,
    numeric_keymap: &HashMap<char, Coord>,
    directional_keymap: &HashMap<char, Coord>,
    cache: &mut HashMap<(Coord, Coord, usize), usize>,
) -> usize {
    if deepth == max_deepth {
        return code.len();
    }
    let keymap = if deepth == 0 {
        numeric_keymap
    } else {
        directional_keymap
    };
    let mut d = 0;
    for (a, b) in repeat(&'A').take(1).chain(code.iter()).zip(code.iter()) {
        let (&a, &b) = (keymap.get(a).unwrap(), keymap.get(b).unwrap());
        if let Some(r) = cache.get(&(a, b, deepth)) {
            d += r;
            continue;
        }
        let dis = (
            (b.0 < a.0) as usize * b.0.abs_diff(a.0),
            (b.0 > a.0) as usize * b.0.abs_diff(a.0),
            (b.1 > a.1) as usize * b.1.abs_diff(a.1),
            (b.1 < a.1) as usize * b.1.abs_diff(a.1),
        );
        let mut r = usize::MAX;
        for mut path in repeat('^')
            .take(dis.0)
            .chain(repeat('v').take(dis.1))
            .chain(repeat('>').take(dis.2))
            .chain(repeat('<').take(dis.3))
            .permutations(dis.0 + dis.1 + dis.2 + dis.3)
        {
            if (deepth == 0 && !test_path(a, &path, &NUMERIC_KEYPAD, NUMERIC_KEYPAD_EMPTY))
                || (deepth != 0
                    && !test_path(a, &path, &DIRECTIONAL_KAYPAD, DIRECTIONAL_KAYPAD_EMPTY))
            {
                continue;
            }
            path.push('A');
            r = r.min(dfs_dp(
                &path,
                deepth + 1,
                max_deepth,
                numeric_keymap,
                directional_keymap,
                cache,
            ));
        }
        cache.insert((a, b, deepth), r);
        d += r;
    }
    d
}

fn part2(codes: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;

    let numeric_keymap = keymap(&NUMERIC_KEYPAD);
    let directional_keymap = keymap(&DIRECTIONAL_KAYPAD);
    let mut cache = HashMap::new();
    for code in codes {
        let r = dfs_dp(
            code,
            0,
            26,
            &numeric_keymap,
            &directional_keymap,
            &mut cache,
        );
        result += r * complexity(code);
    }

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
    assert_eq!(part2(&codes)?, 154115708116294);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let codes = parse_input(input)?;
    assert_eq!(part1(&codes)?, 219366);
    assert_eq!(part2(&codes)?, 271631192020464);
    Ok(())
}
