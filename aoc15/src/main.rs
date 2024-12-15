use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    if let Some((map, moves)) = input.as_ref().trim().split_once("\n\n") {
        let map = map
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect();
        let moves = moves
            .trim()
            .chars()
            .filter(|c| ['<', '>', '^', 'v'].contains(c))
            .map(|c| Move::new(c))
            .collect::<Result<Vec<_>>>()?;
        Ok((map, moves))
    } else {
        err!("unable to parse input")
    }
}

impl Move {
    fn new(c: char) -> Result<Self> {
        match c {
            '>' => Ok(Move::Right),
            '<' => Ok(Move::Left),
            '^' => Ok(Move::Up),
            'v' => Ok(Move::Down),
            _ => err!("{c:?} is not a valid move"),
        }
    }

    fn next_coord(&self, coord: Coord) -> Coord {
        let (x, y) = coord;
        match self {
            Move::Up => (x - 1, y),
            Move::Down => (x + 1, y),
            Move::Left => (x, y - 1),
            Move::Right => (x, y + 1),
        }
    }

    fn move_robot(&self, coord: Coord, map: &mut [Vec<char>]) -> Option<Coord> {
        let (nx, ny) = self.next_coord(coord);
        let (bx, by) = (map.len(), map[0].len());
        if nx < 0 || ny < 0 || nx >= bx as isize || ny >= by as isize {
            None
        } else {
            let (x, y) = (coord.0 as usize, coord.1 as usize);
            let (nx, ny) = (nx as usize, ny as usize);
            if map[nx][ny] == '.'
                || (map[nx][ny] == 'O' && self.move_robot(self.next_coord(coord), map).is_some())
            {
                let t = map[nx][ny];
                map[nx][ny] = map[x][y];
                map[x][y] = t;

                Some((nx as isize, ny as isize))
            } else {
                None
            }
        }
    }

    fn try_push(&self, coord: Coord, map: &[Vec<char>], possible_boxs: &mut Vec<Coord>) -> bool {
        // only conside left side of a box
        let coord = find_box(coord, map);
        let next = self.next_coord(coord);
        let mut push_able = false;
        match self {
            Move::Up | Move::Down => {
                let (x, y) = (next.0 as usize, next.1 as usize);
                if map[x][y] == '.' && map[x][y + 1] == '.' {
                    push_able = true
                } else if map[x][y] == '#' || map[x][y + 1] == '#' {
                    push_able = false
                } else {
                    match (map[x][y], map[x][y + 1]) {
                        ('[', ']') => {
                            if self.try_push(next, map, possible_boxs) {
                                push_able = true
                            }
                        }
                        (']', '[') => {
                            if self.try_push(next, map, possible_boxs)
                                && self.try_push((next.0, next.1 + 1), map, possible_boxs)
                            {
                                push_able = true
                            }
                        }
                        ('.', '[') => {
                            if self.try_push((next.0, next.1 + 1), map, possible_boxs) {
                                push_able = true
                            }
                        }
                        (']', '.') => {
                            if self.try_push(next, map, possible_boxs) {
                                push_able = true
                            }
                        }
                        _ => unreachable!("impossible pattern {:?} {:?}", map[x][y], map[x][y + 1]),
                    }
                }
            }
            Move::Left | Move::Right => {
                let possible = if self == &Move::Left {
                    next
                } else {
                    self.next_coord(next)
                };
                match map[possible.0 as usize][possible.1 as usize] {
                    '.' => push_able = true,
                    '@' => push_able = false,
                    '#' => push_able = false,
                    '[' | ']' => push_able = self.try_push(possible, map, possible_boxs),
                    _ => unreachable!("unknow char at {:?} for map", possible),
                }
            }
        }
        if push_able {
            possible_boxs.push(coord);
        }
        push_able
    }

    fn move_robot_expanded_map(&self, robot: Coord, map: &mut [Vec<char>]) -> Option<Coord> {
        let (nx, ny) = self.next_coord(robot);
        let (bx, by) = (map.len(), map[0].len());
        if nx < 0 || ny < 0 || nx >= bx as isize || ny >= by as isize {
            None
        } else {
            let (x, y) = (robot.0 as usize, robot.1 as usize);
            let (nx, ny) = (nx as usize, ny as usize);
            match map[nx][ny] {
                '@' | '.' => {
                    map[nx][ny] = '@';
                    map[x][y] = '.';
                    Some((nx as isize, ny as isize))
                }
                '#' => None,
                '[' | ']' => {
                    let mut boxs = vec![];
                    if self.try_push(self.next_coord(robot), map, &mut boxs) {
                        for &b in &boxs {
                            let (x, y) = (b.0 as usize, b.1 as usize);
                            map[x][y] = '.';
                            map[x][y + 1] = '.';
                        }
                        for b in boxs {
                            let b = self.next_coord(b);
                            let (x, y) = (b.0 as usize, b.1 as usize);
                            map[x][y] = '[';
                            map[x][y + 1] = ']';
                        }
                        map[nx][ny] = '@';
                        map[x][y] = '.';
                        Some((nx as isize, ny as isize))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

fn find_box(coord: Coord, map: &[Vec<char>]) -> Coord {
    let (x, y) = (coord.0 as usize, coord.1 as usize);
    if map[x][y] == '[' {
        coord
    } else if map[x][y] == ']' {
        (coord.0, coord.1 - 1)
    } else {
        unreachable!("{:?} is not a box", map[x][y]);
    }
}

#[allow(dead_code)]
fn display_map(map: &[Vec<char>]) {
    for line in map.iter().map(|row| row.iter().collect::<String>()) {
        println!("{line}")
    }
}

fn find_robot(map: &[Vec<char>]) -> Coord {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'@' {
                return (i as isize, j as isize);
            }
        }
    }
    unreachable!("there is no robot in map")
}

fn sum_of_gps(map: &[Vec<char>]) -> usize {
    let mut s = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'O' || c == &'[' {
                s += 100 * i + j;
            }
        }
    }
    s
}

fn part1(map: &[Vec<char>], moves: &[Move]) -> Result<usize> {
    let _start = Instant::now();

    let mut map = map.to_vec();
    let mut robot = find_robot(&map);

    for m in moves {
        if let Some(new_robot) = m.move_robot(robot, &mut map) {
            robot = new_robot;
        }
    }

    // display_map(&map);

    let result = sum_of_gps(&map);
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn expand_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = vec![vec!['.'; map[0].len() * 2]; map.len()];

    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                new_map[i][j * 2] = '#';
                new_map[i][j * 2 + 1] = '#';
            } else if c == 'O' {
                new_map[i][j * 2] = '[';
                new_map[i][j * 2 + 1] = ']';
            } else if c == '@' {
                new_map[i][j * 2] = '@';
            }
        }
    }

    new_map
}

fn part2(map: &[Vec<char>], moves: &[Move]) -> Result<usize> {
    let _start = Instant::now();

    let mut map = expand_map(map);
    let mut robot = find_robot(&map);

    // display_map(&map);
    for m in moves {
        if let Some(new_robot) = m.move_robot_expanded_map(robot, &mut map) {
            robot = new_robot;
        }
    }
    display_map(&map);

    let result = sum_of_gps(&map);
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (map, moves) = parse_input(input)?;
    part1(&map, &moves)?;
    part2(&map, &moves)?;
    Ok(())
}

#[test]
fn example_input1() -> Result<()> {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    let (map, moves) = parse_input(input)?;
    assert_eq!(part1(&map, &moves)?, 2028);
    assert_eq!(part2(&map, &moves)?, 1751);
    Ok(())
}

#[test]
fn example_input2() -> Result<()> {
    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    let (map, moves) = parse_input(input)?;
    assert_eq!(part2(&map, &moves)?, 618);
    Ok(())
}

#[test]
fn example_input3() -> Result<()> {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    let (map, moves) = parse_input(input)?;
    assert_eq!(part1(&map, &moves)?, 10092);
    assert_eq!(part2(&map, &moves)?, 9021);
    Ok(())
}
#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (map, moves) = parse_input(input)?;
    assert_eq!(part1(&map, &moves)?, 1538871);
    assert_eq!(part2(&map, &moves)?, 1543338);
    Ok(())
}
