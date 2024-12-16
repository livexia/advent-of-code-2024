use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }

    fn counterclockwise(self) -> Self {
        self.clockwise().clockwise().clockwise()
    }
}

type Coord = (isize, isize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Reindeer {
    facing: Direction,
    coord: Coord,
}

impl Reindeer {
    fn new(coord: Coord) -> Self {
        Reindeer {
            facing: Direction::East,
            coord,
        }
    }
}

type Map = HashMap<Coord, char>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<Map> {
    let mut map = Map::new();
    for (i, row) in input.as_ref().trim().lines().enumerate() {
        for (j, c) in row.trim().chars().enumerate() {
            if c != '#' {
                map.insert((i as isize, j as isize), c);
            }
        }
    }
    Ok(map)
}

fn find_from_map(map: &Map, target: char) -> Option<Coord> {
    for (coord, c) in map {
        if c == &target {
            return Some(*coord);
        }
    }
    None
}

impl Reindeer {
    fn next(&self, map: &Map) -> Option<Self> {
        let (x, y) = self.coord;
        let new_c = match self.facing {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::West => (x, y - 1),
            Direction::East => (x, y + 1),
        };
        if map.contains_key(&new_c) {
            Some(Reindeer {
                facing: self.facing,
                coord: new_c,
            })
        } else {
            None
        }
    }

    fn rotate(&self) -> [Self; 2] {
        [
            Self {
                facing: self.facing.clockwise(),
                coord: self.coord,
            },
            Self {
                facing: self.facing.counterclockwise(),
                coord: self.coord,
            },
        ]
    }

    fn min_score_to(
        &self,
        score: usize,
        min_score: &mut usize,
        map: &Map,
        target: char,
        searching: &mut HashSet<Reindeer>,
    ) {
        if map.get(&self.coord) == Some(&target) {
            *min_score = score.min(*min_score);
        }
        if let Some(n) = self.next(map) {
            if score + 1 < *min_score && !searching.contains(&n) {
                searching.insert(n);
                n.min_score_to(score + 1, min_score, map, target, searching);
                searching.remove(&n);
            }
        }
        for n in self.rotate() {
            if score + 1000 < *min_score && !searching.contains(&n) {
                searching.insert(n);
                n.min_score_to(score + 1000, min_score, map, target, searching);
                searching.remove(&n);
            }
        }
    }
}

#[allow(dead_code)]
fn part1(map: &Map) -> Result<usize> {
    let _start = Instant::now();

    let reindeer = Reindeer::new(find_from_map(map, 'S').unwrap());
    println!("{:?}", reindeer);

    let mut searching = HashSet::new();
    searching.insert(reindeer);
    let mut result = usize::MAX;
    reindeer.min_score_to(0, &mut result, map, 'E', &mut searching);
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part1_dijkstra(map: &Map) -> Result<usize> {
    let _start = Instant::now();

    let reindeer = Reindeer::new(find_from_map(map, 'S').unwrap());
    let mut distance = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, reindeer)));

    while let Some(Reverse((s, r))) = queue.pop() {
        assert_ne!(s, usize::MAX);
        if let Some(next) = r.next(map) {
            let s = s + 1;
            let d = distance.entry(next).or_insert(usize::MAX);
            if s < *d {
                *d = s;
                queue.push(Reverse((s, next)));
            }
        }
        for next in r.rotate() {
            let s = s + 1000;
            let d = distance.entry(next).or_insert(usize::MAX);
            if s < *d {
                *d = s;
                queue.push(Reverse((s, next)));
            }
        }
    }

    let result = distance
        .iter()
        .filter(|(k, _)| k.coord == find_from_map(map, 'E').unwrap())
        .map(|(_, v)| *v)
        .min()
        .unwrap();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input)?;
    // origin method way too slow
    // part1(&map)?;
    part1_dijkstra(&map)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input0() -> Result<()> {
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    let map = parse_input(input)?;
    assert_eq!(part1(&map)?, 7036);
    assert_eq!(part1_dijkstra(&map)?, 7036);
    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn example_input1() -> Result<()> {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    let map = parse_input(input)?;
    assert_eq!(part1_dijkstra(&map)?, 11048);
    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input)?;
    assert_eq!(part1_dijkstra(&map)?, 72400);
    Ok(())
}
