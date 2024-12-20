use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (usize, usize);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Vec<char>>> {
    Ok(input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect())
}

fn find_start_end(map: &[Vec<char>]) -> (Coord, Coord) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end)
}

fn next_coord(x: usize, y: usize, dx: isize, dy: isize, map: &[Vec<char>]) -> Option<Coord> {
    if x as isize + dx < 0
        || x as isize + dx >= map.len() as isize
        || y as isize + dy < 0
        || y as isize + dy >= map[0].len() as isize
    {
        return None;
    }
    Some(((x as isize + dx) as usize, (y as isize + dy) as usize))
}

// find cheats with pathfinding very inefficient
#[allow(dead_code)]
fn find_all_cheats(map: &[Vec<char>], cheat_length: usize) -> HashMap<Coord, HashSet<Coord>> {
    let mut cheats: HashMap<Coord, HashSet<Coord>> = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '#' {
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                queue.push_back(((i, j), 0));
                while let Some((cur, time)) = queue.pop_front() {
                    if visited.insert(cur) && time < cheat_length {
                        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (x, y) = cur;
                            if let Some((nx, ny)) = next_coord(x, y, dx, dy, map) {
                                queue.push_back(((nx, ny), time + 1));
                            }
                        }
                    }
                }
                cheats.insert(
                    (i, j),
                    visited
                        .into_iter()
                        .filter(|(x, y)| map[*x][*y] != '#')
                        .collect(),
                );
            }
        }
    }

    cheats
}

fn p_space(cur: Coord, length: usize, map: &[Vec<char>]) -> Vec<Coord> {
    let mut r = Vec::with_capacity(length * length);

    let length = length as isize;
    for dx in -length..=length {
        for dy in -length..=length {
            if let Some(next) = next_coord(cur.0, cur.1, dx, dy, map) {
                if next.0.abs_diff(cur.0) + next.1.abs_diff(cur.1) > length as usize {
                    continue;
                }
                if map[next.0][next.1] != '#' {
                    r.push(next);
                }
            }
        }
    }
    r
}

fn shortest_path(start: Coord, map: &[Vec<char>]) -> Vec<Vec<usize>> {
    let mut distance = vec![vec![usize::MAX; map[0].len()]; map.len()];

    distance[start.0][start.1] = 0;
    let mut cur = start;
    let mut prev = start;
    let mut time = 0;
    // there is no branch on the racetrack
    while map[cur.0][cur.1] != 'E' {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = cur;
            if let Some(next) = next_coord(x, y, dx, dy, map) {
                if map[next.0][next.1] != '#' && next != prev {
                    prev = cur;
                    cur = next;
                    time += 1;
                    distance[next.0][next.1] = time;
                    break;
                }
            }
        }
    }

    distance
}

fn find_cheates_at_least_save(map: &[Vec<char>], least_save: usize, cheat_length: usize) -> usize {
    let (start, end) = find_start_end(map);
    let s_dis = shortest_path(start, map);
    let origin = s_dis[end.0][end.1];

    // let mut result = 0;
    // for (i, row) in map.iter().enumerate() {
    //     for (j, &c) in row.iter().enumerate() {
    //         if c != '#' {
    //             for next in p_space((i, j), cheat_length, map) {
    //                 if origin
    //                     >= least_save
    //                         + i.abs_diff(next.0)
    //                         + j.abs_diff(next.1)
    //                         + s_dis[i][j]
    //                         + origin
    //                         - s_dis[next.0][next.1]
    //                 {
    //                     result += 1;
    //                 }
    //             }
    //         }
    //     }
    // }

    // result

    map.par_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.par_iter()
                .enumerate()
                .filter(|&(_, c)| *c != '#')
                .map(move |(j, _)| (i, j))
                .map(|(i, j)| {
                    p_space((i, j), cheat_length, map)
                        .par_iter()
                        .filter(|next| {
                            origin
                                >= least_save
                                    + i.abs_diff(next.0)
                                    + j.abs_diff(next.1)
                                    + s_dis[i][j]
                                    + origin
                                    - s_dis[next.0][next.1]
                        })
                        .count()
                })
        })
        .sum()
}

fn part1(map: &[Vec<char>], least_save: usize) -> Result<usize> {
    let _start = Instant::now();

    let result = find_cheates_at_least_save(map, least_save, 2);
    println!("part1: {result}");

    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(map: &[Vec<char>], least_save: usize) -> Result<usize> {
    let _start = Instant::now();

    let result = find_cheates_at_least_save(map, least_save, 20);
    println!("part2: {result}");

    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input)?;
    part1(&map, 100)?;
    part2(&map, 100)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    let map = parse_input(input)?;
    assert_eq!(part1(&map, 40)?, 2);
    assert_eq!(part1(&map, 64)?, 1);
    assert_eq!(part1(&map, 12)?, 8);
    assert_eq!(part1(&map, 2)?, 44);
    assert_eq!(part2(&map, 74)?, 7);
    assert_eq!(part2(&map, 76)?, 3);
    assert_eq!(part2(&map, 72)?, 29);
    assert_eq!(find_cheates_at_least_save(&map, 70, 20), 41);
    assert_eq!(find_cheates_at_least_save(&map, 68, 20), 55);
    assert_eq!(find_cheates_at_least_save(&map, 66, 20), 67);
    assert_eq!(find_cheates_at_least_save(&map, 64, 20), 86);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input)?;
    assert_eq!(part1(&map, 100)?, 1499);
    assert_eq!(part2(&map, 100)?, 1027164);
    Ok(())
}
