use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;
use std::usize;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type OrderingRules = HashMap<usize, HashSet<usize>>;
type Update = Vec<usize>;

fn parse_input<T: AsRef<str>>(input: T) -> (OrderingRules, Vec<Update>) {
    let mut rules = vec![];
    let mut updates = vec![];
    for line in input.as_ref().lines() {
        if let Some((l, r)) = line.trim().split_once('|') {
            rules.push((
                l.trim().parse::<usize>().unwrap(),
                r.trim().parse::<usize>().unwrap(),
            ));
        } else if line.contains(',') {
            updates.push(
                line.trim()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect(),
            );
        } else {
            continue;
        }
    }
    let mut rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (a, b) in rules {
        rules_map.entry(a).or_default().insert(b);
    }
    (rules_map, updates)
}

fn build_ordering_rules(rules: &OrderingRules, update: &Update) -> OrderingRules {
    let mut ordering_rules = OrderingRules::new();

    for &cur in update {
        let e = ordering_rules.entry(cur).or_default();
        find_after(rules, update, cur, e)
    }

    fn find_after(
        rules: &OrderingRules,
        update: &Update,
        cur: usize,
        all_after: &mut HashSet<usize>,
    ) {
        if let Some(after) = rules.get(&cur) {
            for &next in after.iter().filter(|n| update.contains(n)) {
                if all_after.insert(next) {
                    find_after(rules, update, next, all_after);
                }
            }
        }
    }

    ordering_rules
}

fn find(rules: &OrderingRules, cur: usize, target: usize) -> bool {
    if let Some(after) = rules.get(&cur) {
        return after.contains(&target);
    }
    false
}

fn part1(rules: &OrderingRules, updates: &[Update]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = updates.iter().map(|u| u[u.len() / 2]).sum();

    for update in updates {
        let ordering_rules = build_ordering_rules(rules, update);
        for w in update.windows(2) {
            let (a, b) = (w[0], w[1]);
            if find(&ordering_rules, b, a) {
                result -= update[update.len() / 2];
                break;
            }
        }
    }

    println!("part1 :{result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(rules: &OrderingRules, updates: &[Update]) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;

    for update in updates {
        let mut update = update.clone();
        let ordering_rules = build_ordering_rules(rules, &update);
        let (mut cur, mut next) = (0, 1);
        let mut swaped = false;
        while next < update.len() {
            let (a, b) = (update[cur], update[next]);
            if find(&ordering_rules, b, a) {
                update.swap(cur, next);
                swaped = true;
                if cur != 0 {
                    cur -= 1;
                    next -= 1;
                }
                continue;
            }
            cur += 1;
            next += 1;
        }
        if swaped {
            result += update[update.len() / 2];
        }
    }

    println!("part2 :{result}");
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (rules, updates) = parse_input(input);
    part1(&rules, &updates)?;
    part2(&rules, &updates)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let (rules, updates) = parse_input(input);
    assert_eq!(part1(&rules, &updates).unwrap(), 143);
    assert_eq!(part2(&rules, &updates).unwrap(), 123);
    assert_eq!(1, 1);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (rules, updates) = parse_input(input);
    assert_eq!(part1(&rules, &updates).unwrap(), 5129);
    assert_eq!(part2(&rules, &updates).unwrap(), 4077);
}
