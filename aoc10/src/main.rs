use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type TopoMap = Vec<Vec<u8>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<TopoMap> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().bytes().map(|b| b - b'0').collect())
        .collect())
}

fn find_trailheads(map: &TopoMap) -> Vec<(usize, usize)> {
    let mut trailheads = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if height == &0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn find_hiking_trail(coord: (usize, usize), map: &TopoMap) -> Vec<(usize, usize)> {
    let (x, y) = coord;
    let height = map[x][y];
    if height == 9 {
        vec![(x, y)]
    } else {
        let mut result = vec![];
        if x > 0 && map[x - 1][y] == height + 1 {
            result.extend(find_hiking_trail((x - 1, y), map).iter())
        }
        if x + 1 < map.len() && map[x + 1][y] == height + 1 {
            result.extend(find_hiking_trail((x + 1, y), map).iter())
        }
        if y > 0 && map[x][y - 1] == height + 1 {
            result.extend(find_hiking_trail((x, y - 1), map).iter())
        }
        if y + 1 < map[0].len() && map[x][y + 1] == height + 1 {
            result.extend(find_hiking_trail((x, y + 1), map).iter())
        }
        result
    }
}

fn part1(map: &TopoMap) -> Result<usize> {
    let _start = Instant::now();

    let trailheads = find_trailheads(map);
    let result = trailheads
        .iter()
        .map(|&c| {
            find_hiking_trail(c, map)
                .iter()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!(
        "{}",
        trailheads
            .iter()
            .map(|&c| find_hiking_trail(c, map).len())
            .sum::<usize>()
    );

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(map: &TopoMap) -> Result<usize> {
    let _start = Instant::now();

    let trailheads = find_trailheads(map);
    let result = trailheads
        .iter()
        .map(|&c| find_hiking_trail(c, map).len())
        .sum::<usize>();

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input)?;
    part1(&map)?;
    part2(&map)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 36);
    assert_eq!(part2(&map)?, 81);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 566);
    assert_eq!(part2(&map)?, 1324);
    assert_eq!(2, 2);
    Ok(())
}
