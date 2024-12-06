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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
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
                self.facing = self.facing.turn();
                self.patrol(grid)
            }
            None => false,
            _ => unreachable!("There is something wrong with grid at {next_pos:?}"),
        }
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
    let mut guard = guard.clone();
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

fn part2(grid: &Grid, guard: &Guard) -> Result<usize> {
    let _start = Instant::now();

    let patrol_route = patrol_route(grid, guard);

    let mut result = 0;
    let mut grid = grid.clone();
    for (i, j) in patrol_route {
        let (i, j) = (i as usize, j as usize);
        if grid[i][j] == '.' {
            grid[i][j] = '#';

            let mut guard = guard.clone();
            let mut visited: HashSet<Guard> = HashSet::new();
            visited.insert(guard.clone());
            while guard.patrol(&grid) {
                if !visited.insert(guard.clone()) {
                    result += 1;
                    break;
                }
            }

            grid[i][j] = '.';
        }
    }
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
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (grid, guard) = parse_input(input).unwrap();
    assert_eq!(part1(&grid, &guard).unwrap(), 5551);
    assert_eq!(part2(&grid, &guard).unwrap(), 1939);
}
