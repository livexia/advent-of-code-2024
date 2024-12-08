use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (i32, i32);
type Freq = char;
type Map = HashMap<Coord, Freq>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Map, Coord)> {
    let input: Vec<Vec<_>> = input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let bound = (input.len(), input[0].len());
    let mut map = Map::new();
    (0..bound.0).for_each(|i| {
        for j in 0..bound.1 {
            if input[i][j] != '.' {
                map.insert((i as i32, j as i32), input[i][j]);
            }
        }
    });
    Ok((map, (bound.0 as i32, bound.1 as i32)))
}

fn map_to_freq_coords(map: &Map) -> HashMap<Freq, Vec<Coord>> {
    let mut freqs: HashMap<char, Vec<Coord>> = HashMap::new();
    for (&c, &f) in map {
        freqs.entry(f).or_default().push(c);
    }

    freqs
}

fn in_bound(c: Coord, bound: Coord) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < bound.0 && c.1 < bound.1
}

fn find_antinodes(a: Coord, b: Coord) -> [Coord; 2] {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let (x, y) = (a.0, a.1);
    // (x + dx, y + dy) => b
    [(x - dx, y - dy), (x + 2 * dx, y + 2 * dy)]
}

fn part1(map: &Map, bound: Coord) -> Result<usize> {
    let _start = Instant::now();

    let freqs = map_to_freq_coords(map);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in freqs.values() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let a = coords[i];
                let b = coords[j];
                for c in find_antinodes(a, b) {
                    if in_bound(c, bound) {
                        antinodes.insert(c);
                    }
                }
            }
        }
    }

    let result = antinodes.len();

    println!("part1: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn find_antinodes_part2(a: Coord, b: Coord, bound: Coord) -> Vec<Coord> {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let (x, y) = a;

    (0..)
        .map(|i| (x - i * dx, y - i * dy))
        .take_while(|&c| in_bound(c, bound))
        .chain(
            (0..)
                .map(|i| (x + i * dx, y + i * dy))
                .take_while(|&c| in_bound(c, bound)),
        )
        .collect()
}

fn part2(map: &Map, bound: Coord) -> Result<usize> {
    let _start = Instant::now();

    let freqs = map_to_freq_coords(map);
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in freqs.values() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let a = coords[i];
                let b = coords[j];
                antinodes.extend(find_antinodes_part2(a, b, bound).iter());
            }
        }
    }

    let result = antinodes.len();

    println!("part2: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (map, bound) = parse_input(input)?;
    part1(&map, bound)?;
    part2(&map, bound)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    let (map, bound) = parse_input(input)?;
    assert_eq!(part1(&map, bound)?, 14);
    assert_eq!(part2(&map, bound)?, 34);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (map, bound) = parse_input(input)?;
    assert_eq!(part1(&map, bound)?, 359);
    assert_eq!(part2(&map, bound)?, 1293);
    Ok(())
}
