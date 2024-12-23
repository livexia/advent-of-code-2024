use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read};
use std::time::Instant;

use itertools::Itertools;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Idx = HashMap<usize, String>;
type Network = HashMap<usize, HashSet<usize>>;

fn parse_input<T: AsRef<str>>(input: T) -> Result<(Network, Idx)> {
    fn try_insert_with_id(idx: &mut HashMap<String, usize>, id: &mut usize, item: &str) -> usize {
        if let Some(id) = idx.get(item) {
            *id
        } else {
            idx.insert(item.to_string(), *id);
            *id += 1;
            *id - 1
        }
    }
    let mut network = Network::new();
    let mut idx_name = HashMap::new();
    let mut idx_id = Idx::new();
    let mut id = 0;
    for line in input.as_ref().trim().lines() {
        if let Some((l, r)) = line.trim().split_once("-") {
            let l_id = try_insert_with_id(&mut idx_name, &mut id, l);
            let r_id = try_insert_with_id(&mut idx_name, &mut id, r);
            idx_id.insert(l_id, l.to_string());
            idx_id.insert(r_id, r.to_string());
            network.entry(l_id).or_default().insert(r_id);
            network.entry(r_id).or_default().insert(l_id);
        }
    }
    Ok((network, idx_id))
}

fn three_inter_connected(id: usize, network: &Network) -> Vec<[usize; 3]> {
    let mut parties = vec![];
    for &connected in network.get(&id).unwrap().iter().filter(|&&c| c != id) {
        for &last in network
            .get(&connected)
            .unwrap_or(&HashSet::new())
            .iter()
            .filter(|&&c| c != id && c != connected)
        {
            if network.get(&id).unwrap().contains(&last) {
                parties.push([id, connected, last]);
            }
        }
    }
    parties
}

fn start_with_t(party: &[usize], idx: &Idx) -> bool {
    party.iter().any(|id| idx.get(id).unwrap().starts_with('t'))
}

fn part1(network: &Network, idx: &Idx) -> Result<usize> {
    let _start = Instant::now();

    let mut historian = HashSet::new();
    for &id in network.keys() {
        for mut party in three_inter_connected(id, network) {
            party.sort();
            if start_with_t(&party, idx) {
                historian.insert(party);
            }
        }
    }

    let result = historian.len();
    println!("part1: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn perfect_lan_party(id: usize, network: &Network) -> Option<HashSet<usize>> {
    let connected = network.get(&id).unwrap();
    let mut perfect = connected.clone();
    perfect.insert(id);

    for i in (2..perfect.len()).rev() {
        for party in perfect.iter().cloned().combinations(i) {
            let party: HashSet<_> = party.into_iter().collect();
            if is_perfect(&party, network) {
                return Some(party);
            }
        }
    }
    None
}

fn is_perfect(party: &HashSet<usize>, network: &Network) -> bool {
    for &id in party {
        let mut s = network.get(&id).unwrap().clone();
        s.insert(id);
        if !party.is_subset(&s) {
            return false;
        }
    }
    true
}

fn part2(network: &Network, idx: &Idx) -> Result<String> {
    let _start = Instant::now();

    let mut lan_party = Vec::new();

    for &id in idx.keys() {
        if let Some(party) = perfect_lan_party(id, network) {
            if party.len() >= lan_party.len() {
                let mut party: Vec<_> = party
                    .iter()
                    .map(|id| idx.get(id).unwrap().clone())
                    .collect();
                party.sort();
                lan_party = party;
            }
        }
    }

    let result = lan_party.join(",");
    println!("part2: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn bron_kerbosch_algo(
    clique: HashSet<usize>,
    some_vertices: &mut HashSet<usize>,
    none_vertices: &mut HashSet<usize>,
    network: &Network,
    maximal_cliques: &mut Vec<HashSet<usize>>,
) {
    if some_vertices.is_empty() && none_vertices.is_empty() {
        // find a maximal clique
        maximal_cliques.push(clique);
        return;
    }
    while let Some(id) = some_vertices.iter().copied().next() {
        // for id in some_vertices.iter().copied() {
        let neighbor = network.get(&id).unwrap();
        let mut p1: HashSet<_> = some_vertices.intersection(neighbor).copied().collect();
        let mut x1: HashSet<_> = none_vertices.intersection(neighbor).copied().collect();
        let mut clique = clique.clone();
        clique.insert(id);
        bron_kerbosch_algo(clique, &mut p1, &mut x1, network, maximal_cliques);
        some_vertices.remove(&id);
        none_vertices.insert(id);
    }
}

fn part2_with_bron_kerbosch(network: &Network, idx: &Idx) -> Result<String> {
    let _start = Instant::now();

    let mut maximal_cliques = vec![];
    bron_kerbosch_algo(
        HashSet::new(),
        &mut network.keys().copied().collect(),
        &mut HashSet::new(),
        network,
        &mut maximal_cliques,
    );

    let mut party: Vec<_> = maximal_cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .map(|id| idx.get(id).unwrap().clone())
        .collect();
    party.sort();
    let result = party.join(",");
    println!("part2 with Bron–Kerbosch algorithm: {result}");
    println!("> Time elapsed is: {:?}", _start.elapsed());
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (network, idx) = parse_input(input)?;
    part1(&network, &idx)?;
    part2(&network, &idx)?;
    part2_with_bron_kerbosch(&network, &idx)?;
    Ok(())
}

#[test]
fn example_input() -> Result<()> {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    let (network, idx) = parse_input(input)?;
    assert_eq!(part1(&network, &idx)?, 7);
    assert_eq!(part2(&network, &idx)?, "co,de,ka,ta".to_string());
    assert_eq!(
        part2_with_bron_kerbosch(&network, &idx)?,
        "co,de,ka,ta".to_string()
    );
    Ok(())
}

#[test]
fn real_input() -> Result<()> {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (network, idx) = parse_input(input)?;
    assert_eq!(part1(&network, &idx)?, 1064);
    assert_eq!(
        part2(&network, &idx)?,
        "aq,cc,ea,gc,jo,od,pa,rg,rv,ub,ul,vr,yy"
    );
    assert_eq!(
        part2_with_bron_kerbosch(&network, &idx)?,
        "aq,cc,ea,gc,jo,od,pa,rg,rv,ub,ul,vr,yy"
    );
    assert_eq!(2, 2);
    Ok(())
}
