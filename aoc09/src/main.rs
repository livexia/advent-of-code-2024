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
#[allow(dead_code)]
struct DiskMap {
    raw: Vec<usize>,
    free: Vec<(usize, usize)>,
    files: Vec<(usize, usize, usize)>,
}

impl FromStr for DiskMap {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut raw = vec![];
        let mut free = vec![];
        let mut files = vec![];
        let mut is_free = false;
        for (i, b) in s.trim().bytes().enumerate() {
            let b = (b - b'0') as usize;
            let id = match is_free {
                true => {
                    free.push((raw.len(), raw.len() + b));
                    usize::MAX
                }
                false => {
                    files.push((raw.len(), raw.len() + b, i / 2));
                    i / 2
                }
            };
            raw.extend(iter::repeat(id).take(b));
            is_free = !is_free;
        }
        Ok(Self { raw, free, files })
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

    fn compact_file_fragment(&mut self) {
        for i in (0..self.files.len()).rev() {
            for free in &mut self.free {
                let file = self.files[i];
                let free_size = free.1 - free.0;
                let file_size = file.1 - file.0;
                if free.0 >= file.0 {
                    break;
                } else if free_size >= file_size && free.0 < file.0 {
                    self.files[i] = (free.0, free.0 + file_size, file.2);
                    free.0 += file_size;
                    break;
                } else if free_size != 0 {
                    self.files[i] = (file.0, file.0 + file_size - free_size, file.2);
                    self.files.push((free.0, free.1, file.2));
                    free.0 = free.1;
                }
            }
        }
    }

    fn compact_whole_file(&mut self) {
        for file in self.files.iter_mut().rev() {
            for free in &mut self.free {
                let size = file.1 - file.0;
                if free.1 - free.0 >= size && free.0 < file.0 {
                    *file = (free.0, free.0 + size, file.2);
                    free.0 += size;
                    break;
                }
            }
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

    fn checksum_files(&self) -> usize {
        self.files
            .iter()
            .cloned()
            .fold(0, |s, (i, j, id)| s + (i..j).sum::<usize>() * id)
    }
}

fn part1(disk_map: &DiskMap) -> Result<usize> {
    let _start = Instant::now();

    let mut disk_map = disk_map.clone();
    disk_map.compact();
    let result = disk_map.checksum();

    println!("part1 with double pointer: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part1_interval(disk_map: &DiskMap) -> Result<usize> {
    let _start = Instant::now();

    let mut disk_map = disk_map.clone();
    disk_map.compact_file_fragment();
    let result = disk_map.checksum_files();

    println!("part1 with interval: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn part2(disk_map: &DiskMap) -> Result<usize> {
    let _start = Instant::now();

    let mut disk_map = disk_map.clone();
    disk_map.compact_whole_file();
    let result = disk_map.checksum_files();

    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let disk_map: DiskMap = input.parse()?;

    part1(&disk_map)?;
    part1_interval(&disk_map)?;
    part2(&disk_map)?;
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
    assert_eq!(part1_interval(&disk_map)?, 1928);
    assert_eq!(part2(&disk_map)?, 2858);
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let disk_map: DiskMap = input.parse()?;

    assert_eq!(part1(&disk_map)?, 6320029754031);
    assert_eq!(part1_interval(&disk_map)?, 6320029754031);
    assert_eq!(part2(&disk_map)?, 6347435485773);
    Ok(())
}
