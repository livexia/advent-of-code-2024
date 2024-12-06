use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Grid = Vec<Vec<char>>;
type Coord = (isize, isize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    facing: Direction,
    coord: Coord,
}

impl Guard {
    fn new(facing: char, coord: Coord) -> Result<Self> {
        let facing = match facing {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => return err!("Unable to parse direction {facing:?} for guard"),
        };
        Ok(Self { facing, coord })
    }

    fn next_position(&self) -> Coord {
        let (x, y) = self.coord;
        match self.facing {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        }
    }

    fn patrol(&mut self, grid: &Grid) -> bool {
        let next_pos = self.next_position();
        match grid_at(grid, next_pos) {
            Some('.') | Some('v') | Some('^') | Some('<') | Some('>') => {
                self.coord = next_pos;
                true
            }
            Some('#') => {
                self.turn();
                self.patrol(grid)
            }
            None => false,
            _ => unreachable!("There is something wrong with grid at {next_pos:?}"),
        }
    }

    fn turn(&mut self) {
        self.facing = self.facing.turn();
    }
}

fn grid_at(grid: &Grid, coord: Coord) -> Option<char> {
    if (0..grid.len() as isize).contains(&coord.0) && (0..grid[0].len() as isize).contains(&coord.1)
    {
        Some(grid[coord.0 as usize][coord.1 as usize])
    } else {
        None
    }
}
fn parse_input<T: AsRef<str>>(input: T) -> Result<(Grid, Guard)> {
    let grid: Grid = input
        .as_ref()
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' && grid[i][j] != '#' {
                let f = grid[i][j];
                return Ok((grid, Guard::new(f, (i as isize, j as isize))?));
            }
        }
    }
    err!("Unable to parse input")
}

fn patrol_route(grid: &Grid, guard: &Guard) -> HashSet<Coord> {
    let mut guard = *guard;
    let mut route = HashSet::new();
    route.insert(guard.coord);

    while guard.patrol(grid) {
        route.insert(guard.coord);
    }

    route
}

fn part1(grid: &Grid, guard: &Guard) -> Result<usize> {
    let _start = Instant::now();

    let result = patrol_route(grid, guard).len();
    println!("part1: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

#[allow(dead_code)]
fn part2_bruteforce_trim(grid: &Grid, guard: &Guard) -> Result<usize> {
    let _start = Instant::now();

    let mut grid = grid.clone();

    let mut result = 0;
    for (x, y) in patrol_route(&grid, guard) {
        if grid_at(&grid, (x, y)) == Some('.') {
            let mut visited = HashSet::new();
            let mut guard = *guard;
            visited.insert(guard);
            grid[x as usize][y as usize] = '#';
            while guard.patrol(&grid) {
                if !visited.insert(guard) {
                    result += 1;
                    break;
                }
            }
            grid[x as usize][y as usize] = '.';
        }
    }

    println!("part2 with bruteforce an trim: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(grid: &Grid, guard: &Guard) -> Result<usize> {
    let _start = Instant::now();

    let mut cyclic = HashSet::new();

    let mut grid = grid.clone();

    let mut guard = *guard;
    let mut alt_guard = guard;
    let mut old_guard = guard;
    let mut visited: HashSet<Guard> = HashSet::with_capacity(grid.len() * grid[0].len());
    let mut checked = HashSet::new();
    while guard.patrol(&grid) {
        if checked.insert(guard.coord) && grid_at(&grid, guard.coord) == Some('.') {
            let (x, y) = (guard.coord.0 as usize, guard.coord.1 as usize);
            grid[x][y] = '#';
            let mut visited = visited.clone();
            visited.insert(alt_guard);
            while alt_guard.patrol(&grid) {
                if !visited.insert(alt_guard) {
                    cyclic.insert(guard.coord);
                    break;
                }
            }
            grid[x][y] = '.';
        }

        visited.insert(old_guard);
        old_guard = guard;
        alt_guard = guard;
    }

    let result = cyclic.len();
    println!("part2: {result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (grid, guard) = parse_input(input)?;
    part1(&grid, &guard)?;
    part2(&grid, &guard)?;
    // part2_bruteforce_trim(&grid, &guard)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    let (grid, guard) = parse_input(input).unwrap();
    assert_eq!(part1(&grid, &guard).unwrap(), 41);
    assert_eq!(part2(&grid, &guard).unwrap(), 6);
}

#[test]
fn test_guard() {
    let input = ".#.
.^#
...";
    let (grid, mut guard) = parse_input(input).unwrap();
    assert_eq!(guard.coord, (1, 1));
    assert_eq!(guard.facing, Direction::Up);
    guard.patrol(&grid);
    assert_eq!(guard.facing, Direction::Down);
    assert_eq!(guard.coord, (2, 1));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (grid, guard) = parse_input(input).unwrap();
    assert_eq!(part1(&grid, &guard).unwrap(), 5551);
    // assert_eq!(part2_bruteforce_trim(&grid, &guard).unwrap(), 1939);
    assert_eq!(part2(&grid, &guard).unwrap(), 1939);
}
