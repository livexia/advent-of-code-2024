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
        if next.is_empty() {
            continue;
        }
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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input)?;
    part1(&map)?;
    part2(&map)?;
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
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 1494342);
    assert_eq!(part2(&map)?, 893676);
    Ok(())
}
