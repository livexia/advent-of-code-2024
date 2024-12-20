use std::collections::{HashMap, VecDeque};
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

fn find_all_cheats(
    map: &[Vec<char>],
    cheat_length: usize,
) -> HashMap<Coord, HashMap<Coord, usize>> {
    let mut cheats: HashMap<Coord, HashMap<Coord, usize>> = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '#' {
                let mut queue = VecDeque::new();
                let mut dis = HashMap::new();
                queue.push_back(((i, j), 0));
                while let Some((cur, time)) = queue.pop_front() {
                    if map[cur.0][cur.1] == '.' && time > 1 {
                        continue;
                    }
                    if time < cheat_length {
                        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let (x, y) = cur;
                            if let Some((nx, ny)) = next_coord(x, y, dx, dy, map) {
                                let t = dis.entry((nx, ny)).or_insert(usize::MAX);
                                if *t > time + 1 {
                                    *t = time + 1;
                                    queue.push_back(((nx, ny), time + 1));
                                }
                            }
                        }
                    }
                }
                cheats.insert(
                    (i, j),
                    dis.into_iter()
                        .filter(|&((x, y), _)| map[x][y] != '#')
                        .collect(),
                );
            }
        }
    }

    cheats
}

fn shortest_path(start: Coord, map: &[Vec<char>]) -> HashMap<Coord, usize> {
    let mut distance = HashMap::new();
    let mut queue = VecDeque::new();

    distance.insert(start, 0);
    queue.push_back((start, 0));
    while let Some((cur, time)) = queue.pop_front() {
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = cur;
            if let Some((nx, ny)) = next_coord(x, y, dx, dy, map) {
                if map[nx][ny] != '#' {
                    let t = distance.entry((nx, ny)).or_insert(usize::MAX);
                    if time + 1 < *t {
                        *t = time + 1;
                        queue.push_back(((nx, ny), time + 1));
                    }
                }
            }
        }
    }
    distance
}

fn find_cheates_at_least_save(map: &[Vec<char>], least_save: usize, cheat_length: usize) -> usize {
    let (start, end) = find_start_end(map);
    let cheats = find_all_cheats(map, cheat_length);
    let s_dis = shortest_path(start, map);
    let e_dis = shortest_path(end, map);

    let mut result = 0;
    let &origin = s_dis.get(&end).unwrap();
    for (s, ns) in cheats {
        for (e, t) in ns {
            let time = t + s_dis.get(&s).unwrap() + e_dis.get(&e).unwrap();
            if origin >= least_save + time {
                result += 1;
            }
        }
    }
    result
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
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    assert_eq!(2, 2);
    Ok(())
}
