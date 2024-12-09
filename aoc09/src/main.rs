use std::error::Error;
use std::io::{self, Read};
use std::iter;
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Clone)]
struct DiskMap {
    raw: Vec<usize>,
}

impl FromStr for DiskMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut raw = vec![];
        let mut is_free = false;
        for (i, b) in s.trim().bytes().enumerate() {
            let b = (b - b'0') as usize;
            let id = match is_free {
                true => usize::MAX,
                false => i / 2,
            };
            raw.extend(iter::repeat(id).take(b));
            is_free = !is_free;
        }
        Ok(Self { raw })
    }
}

impl DiskMap {
    fn compact(&mut self) {
        let mut tail = self.raw.len() - 1;
        let mut head = 0;
        while head < tail && tail != 0 {
            if self.raw[tail] != usize::MAX {
                if self.raw[head] == usize::MAX {
                    self.raw.swap(head, tail);
                    tail -= 1;
                }
                head += 1;
            } else {
                tail -= 1;
            }
        }
    }

    fn find_whole_file(&self, mut tail: usize) -> (usize, usize) {
        let end = tail + 1;
        while tail != 0 && self.raw[tail] == self.raw[end - 1] {
            tail -= 1;
        }
        (tail + 1, end)
    }

    fn find_whole_free_space(&self, mut head: usize) -> (usize, usize) {
        let start = head;
        while head < self.raw.len() && self.raw[head] == usize::MAX {
            head += 1;
        }
        (start, head)
    }

    fn compact_whole_file(&mut self) {
        let mut tail = self.raw.len() - 1;
        while tail != 0 {
            if self.raw[tail] != usize::MAX {
                let file = self.find_whole_file(tail);
                let mut head = 0;
                while head < self.raw.len() {
                    if self.raw[head] == usize::MAX {
                        let free = self.find_whole_free_space(head);
                        if free.0 < file.0 && free.1 - free.0 >= file.1 - file.0 {
                            for i in 0..file.1 - file.0 {
                                self.raw.swap(file.0 + i, free.0 + i);
                            }
                            break;
                        } else {
                            head = free.1;
                        }
                    } else {
                        head += 1;
                    }
                }

                tail = file.0;
            }
            tail -= 1;
        }
    }

    fn checksum(&self) -> usize {
        self.raw.iter().enumerate().fold(
            0,
            |s, (i, &id)| {
                if id == usize::MAX { s } else { s + i * id }
            },
        )
    }
}

fn part1(disk_map: &DiskMap) -> Result<usize> {
    let _start = Instant::now();

    let mut disk_map = disk_map.clone();
    disk_map.compact();
    let result = disk_map.checksum();

    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(disk_map: &DiskMap) -> Result<usize> {
    let _start = Instant::now();

    let mut disk_map = disk_map.clone();
    disk_map.compact_whole_file();
    let result = disk_map.checksum();

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let disk_map: DiskMap = input.parse()?;

    part1(&disk_map)?;
    part2(&disk_map)?;
    Ok(())
}

#[test]
fn simple_example_input() -> Result<()> {
    let input = "12345";
    let mut disk_map: DiskMap = input.parse()?;
    disk_map.compact();

    assert_eq!(disk_map.raw, [
        0,
        2,
        2,
        1,
        1,
        1,
        2,
        2,
        2,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
        usize::MAX,
    ]);
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "2333133121414131402";
    let disk_map: DiskMap = input.parse()?;
    assert_eq!(
        disk_map.raw,
        "00...111...2...333.44.5555.6666.777.888899"
            .bytes()
            .map(|b| if b == b'.' {
                usize::MAX
            } else {
                (b - b'0') as usize
            })
            .collect::<Vec<_>>()
    );

    assert_eq!(part1(&disk_map)?, 1928);
    assert_eq!(part2(&disk_map)?, 2858);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let disk_map: DiskMap = input.parse()?;

    assert_eq!(part1(&disk_map)?, 6320029754031);
    assert_eq!(part2(&disk_map)?, 6347435485773);
    Ok(())
}
