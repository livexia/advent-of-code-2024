use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Coord>> {
    input
        .as_ref()
        .trim()
        .lines()
        .map(|l| {
            if let Some((x, y)) = l.split_once(",") {
                Ok((x.trim().parse()?, y.trim().parse()?))
            } else {
                err!("unable to parse: {l:?}")
            }
        })
        .collect()
}

fn shortest_path(bytes: &[Coord], count: usize, bound: Coord) -> Option<usize> {
    let corrupted: HashSet<_> = bytes[..count].iter().collect();
    let start = (0, 0);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((cur, step)) = queue.pop_front() {
        if cur == bound {
            return Some(step);
        }
        if visited.insert(cur) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (cur.0 + dx, cur.1 + dy);
                if nx < 0 || ny < 0 || nx > bound.0 || ny > bound.1 || corrupted.contains(&(nx, ny))
                {
                    continue;
                }
                queue.push_back(((nx, ny), step + 1));
            }
        }
    }
    None
}

fn part1(bytes: &[Coord], count: usize, bound: Coord) -> Result<usize> {
    let _start = Instant::now();

    let result = shortest_path(bytes, count, bound).unwrap();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn reachable(
    cur: Coord,
    corrupted: &HashSet<Coord>,
    bound: Coord,
    visited: &mut HashSet<Coord>,
) -> bool {
    if cur == bound {
        return true;
    }
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = (cur.0 + dx, cur.1 + dy);
        if nx < 0 || ny < 0 || nx > bound.0 || ny > bound.1 || corrupted.contains(&(nx, ny)) {
            continue;
        }
        if visited.insert((nx, ny)) && reachable((nx, ny), corrupted, bound, visited) {
            return true;
        }
    }
    false
}

fn part2(bytes: &[Coord], count: usize, bound: Coord) -> Result<Coord> {
    let _start = Instant::now();

    let mut result = (0, 0);
    for i in count + 1..=bytes.len() {
        let corrupted: HashSet<_> = bytes[..i].iter().cloned().collect();
        if !reachable((0, 0), &corrupted, bound, &mut HashSet::new()) {
            // if shortest_path(bytes, i, bound).is_none() {
            result = bytes[i - 1];
            break;
        }
    }

    println!("part2: {result:?}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bytes = parse_input(input)?;
    part1(&bytes, 1024, (70, 70))?;
    part2(&bytes, 1024, (70, 70))?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    let bytes = parse_input(input)?;
    assert_eq!(part1(&bytes, 12, (6, 6))?, 22);
    assert_eq!(part2(&bytes, 12, (6, 6))?, (6, 1));
    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let bytes = parse_input(input)?;
    assert_eq!(part1(&bytes, 1024, (70, 70))?, 294);
    assert_eq!(part2(&bytes, 1024, (70, 70))?, (31, 32));
    assert_eq!(2, 2);
    Ok(())
}
