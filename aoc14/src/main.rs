use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (i64, i64);

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<(Coord, Coord)>> {
    let mut robots = vec![];
    for line in input.as_ref().trim().lines() {
        if let Some((l, r)) = line.trim().split_once(' ') {
            let p = if let Some(p) = l.strip_prefix("p=") {
                if let Some((x, y)) = p.trim().split_once(",") {
                    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
                } else {
                    return err!("Unable to parse line: {:?}", line);
                }
            } else {
                return err!("Unable to parse line: {:?}", line);
            };
            let v = if let Some(p) = r.strip_prefix("v=") {
                if let Some((x, y)) = p.trim().split_once(",") {
                    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
                } else {
                    return err!("Unable to parse line: {:?}", line);
                }
            } else {
                return err!("Unable to parse line: {:?}", line);
            };
            robots.push((p, v));
        }
    }
    Ok(robots)
}

fn moving(robot: &mut (Coord, Coord), bound: Coord) {
    let (x, y) = robot.0;
    let (nx, ny) = (x + robot.1.0, y + robot.1.1);
    let (nx, ny) = (nx.rem_euclid(bound.0), ny.rem_euclid(bound.1));
    robot.0 = (nx, ny);
}

fn move_robots(robots: &[(Coord, Coord)], secs: usize, bound: Coord) -> usize {
    let mut robots = robots.to_vec();
    for _i in 0..secs {
        for robot in &mut robots {
            moving(robot, bound);
        }
    }

    let mut quadrant = [0; 4];
    for (p, _) in robots {
        let (x, y) = p;
        if x == bound.0 / 2 || y == bound.1 / 2 {
            continue;
        }
        match (x < bound.0 / 2, y < bound.1 / 2) {
            (true, true) => quadrant[0] += 1,
            (true, false) => quadrant[1] += 1,
            (false, true) => quadrant[2] += 1,
            (false, false) => quadrant[3] += 1,
        }
    }
    quadrant.into_iter().product()
}

fn display_robots(robots: &[(Coord, Coord)], bound: Coord) {
    let mut grid = vec![vec!['.'; bound.1 as usize]; bound.0 as usize];
    for (p, _) in robots {
        grid[p.0 as usize][p.1 as usize] = '*';
    }
    for i in 0..bound.1 as usize {
        (0..bound.0 as usize).for_each(|j| {
            print!("{}", grid[j][i]);
        });
        println!()
    }
}

fn part1(robots: &[(Coord, Coord)]) -> Result<usize> {
    let _start = Instant::now();

    let result = move_robots(robots, 100, (101, 103));
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn dis(a: Coord, b: Coord) -> usize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
}

fn total_dis(robots: &[(Coord, Coord)]) -> usize {
    let mut d = 0;
    for i in 0..robots.len() {
        for j in i + 1..robots.len() {
            d += dis(robots[i].0, robots[j].0)
        }
    }
    d
}

fn part2(robots: &[(Coord, Coord)]) -> Result<usize> {
    let _start = Instant::now();

    let mut robots = robots.to_vec();
    let mut min_dis = usize::MAX;
    let mut last_sec = 0;
    let max_dur = 5000;
    for i in 1.. {
        for robot in &mut robots {
            moving(robot, (101, 103));
        }
        let d = total_dis(&robots);
        if d < min_dis {
            min_dis = d;
            println!("seconds: {i}");
            display_robots(&robots, (101, 103));
            last_sec = i;
        }
        if i - last_sec > max_dur {
            println!("over {max_dur} seconds duration");
            break;
        }
    }

    println!("part2: {last_sec}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(last_sec)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let robots = parse_input(input)?;
    part1(&robots)?;
    part2(&robots)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    let robots = parse_input(input)?;
    assert_eq!(move_robots(&robots, 100, (11, 7)), 12);
    assert_eq!(1, 1);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let robots = parse_input(input)?;
    assert_eq!(part1(&robots)?, 228410028);
    assert_eq!(part2(&robots)?, 8258);
    assert_eq!(2, 2);
    Ok(())
}
