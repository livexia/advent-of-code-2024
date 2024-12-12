use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

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

fn dfs(x: usize, y: usize, map: &[Vec<char>], area: &mut HashSet<(isize, isize)>) -> usize {
    if area.insert((x as isize, y as isize)) {
        let mut perimeter = 0;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if (x == 0 && dx == -1)
                || (x + 1 == map.len() && dx == 1)
                || (y == 0 && dy == -1)
                || (y + 1 == map[0].len() && dy == 1)
            {
                perimeter += 1;
                continue;
            }
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if map[nx][ny] == map[x][y] {
                let p = dfs(nx, ny, map, area);
                perimeter += p;
            } else {
                perimeter += 1;
            }
        }
        perimeter
    } else {
        0
    }
}

fn part1(map: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i as isize, j as isize)) {
                let mut area = HashSet::new();
                let p = dfs(i, j, map, &mut area);
                result += area.len() * p;
                visited.extend(&area);
            }
        }
    }

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn number_of_side(area: &HashSet<(isize, isize)>) -> usize {
    type Coord = (isize, isize);
    let mut side: HashMap<(Coord, Coord), HashSet<Coord>> = HashMap::new();
    for &(x, y) in area.iter() {
        let next: Vec<_> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter(|&(dx, dy)| !area.contains(&(x + dx, y + dy)))
            .collect();
        for (dx, dy) in next {
            let line = (x * dx.abs(), dy * dy.abs());
            side.entry((line, (dx, dy)))
                .or_default()
                .insert((x + dx, y + dy));
        }
    }
    let mut count = 0;
    for points in side.values() {
        let mut visited = HashSet::new();
        for &(x, y) in points {
            if visited.insert((x, y)) {
                count += 1;
                (1..)
                    .map(|i| (x - i, y))
                    .take_while(|c| points.contains(c))
                    .chain((1..).map(|i| (x + i, y)).take_while(|c| points.contains(c)))
                    .chain((1..).map(|i| (x, y - i)).take_while(|c| points.contains(c)))
                    .chain((1..).map(|i| (x, y + i)).take_while(|c| points.contains(c)))
                    .for_each(|c| {
                        visited.insert(c);
                    });
            }
        }
    }
    count
}

fn part2(map: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i as isize, j as isize)) {
                let mut area = HashSet::new();
                let _ = dfs(i, j, map, &mut area);
                result += number_of_side(&area) * area.len();
                visited.extend(&area);
            }
        }
    }

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn next(x: isize, y: isize, north: isize, east: isize) -> (isize, isize) {
    (x + north, y + east)
}

fn is_covex(x: isize, y: isize, north: isize, east: isize, area: &HashSet<(isize, isize)>) -> bool {
    !area.contains(&next(x, y, north, 0)) && !area.contains(&next(x, y, 0, east))
}

fn is_concave(
    x: isize,
    y: isize,
    north: isize,
    east: isize,
    area: &HashSet<(isize, isize)>,
) -> bool {
    !area.contains(&next(x, y, north, east))
        && area.contains(&next(x, y, north, 0))
        && area.contains(&next(x, y, 0, east))
}

fn number_of_corner(area: &HashSet<(isize, isize)>) -> usize {
    let mut count = 0;
    for &(x, y) in area {
        let north = -1;
        let east = 1;
        let south = 1;
        let west = -1;
        // covex
        count += is_covex(x, y, north, east, area) as usize
            + is_covex(x, y, north, west, area) as usize
            + is_covex(x, y, south, east, area) as usize
            + is_covex(x, y, south, west, area) as usize;

        // concave
        count += is_concave(x, y, north, east, area) as usize
            + is_concave(x, y, north, west, area) as usize
            + is_concave(x, y, south, east, area) as usize
            + is_concave(x, y, south, west, area) as usize;
    }
    count
}

fn part2_count_corner(map: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i as isize, j as isize)) {
                let mut area = HashSet::new();
                let _ = dfs(i, j, map, &mut area);
                result += number_of_corner(&area) * area.len();
                visited.extend(&area);
            }
        }
    }

    println!("part2 by count couner: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input)?;
    part1(&map)?;
    part2(&map)?;
    part2_count_corner(&map)?;
    Ok(())
}

#[test]
fn example_input0() -> Result<()> {
    let input = "AAAA
BBCD
BBCC
EEEC";
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 140);
    assert_eq!(part2(&map)?, 80);
    assert_eq!(part2_count_corner(&map)?, 80);
    Ok(())
}

#[test]
fn example_input1() -> Result<()> {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 772);
    assert_eq!(part2(&map)?, 436);
    assert_eq!(part2_count_corner(&map)?, 436);
    Ok(())
}

#[test]
fn example_input2() -> Result<()> {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 1930);
    assert_eq!(part2(&map)?, 1206);
    assert_eq!(part2_count_corner(&map)?, 1206);
    Ok(())
}

#[test]
fn example_input3() -> Result<()> {
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    let map = parse_input(input)?;
    assert_eq!(part2(&map)?, 236);
    assert_eq!(part2_count_corner(&map)?, 236);
    Ok(())
}

#[test]
fn example_input4() -> Result<()> {
    let input = "AAAA";
    let map = parse_input(input)?;
    assert_eq!(part2(&map)?, 16);
    assert_eq!(part2_count_corner(&map)?, 16);
    Ok(())
}

#[test]
fn example_input5() -> Result<()> {
    let input = "A
A
A
A";
    let map = parse_input(input)?;
    assert_eq!(part2(&map)?, 16);
    assert_eq!(part2_count_corner(&map)?, 16);
    Ok(())
}
#[test]
fn example_input6() -> Result<()> {
    let input = "OOO
OXO
OOO
OXO
OOO";
    let map = parse_input(input)?;
    assert_eq!(part2(&map)?, 164);
    assert_eq!(part2_count_corner(&map)?, 164);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 1494342);
    assert_eq!(part2(&map)?, 893676);
    assert_eq!(part2_count_corner(&map)?, 893676);
    Ok(())
}
