use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<String>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split("-").map(|x| x.to_string()).collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use rustc_hash::{FxHashMap, FxHashSet};

#[aoc(day23, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut connections = FxHashMap::default();
    for connection in x {
        connections
            .entry(connection[0].clone())
            .or_insert(FxHashSet::default())
            .insert(connection[1].clone());
        connections
            .entry(connection[1].clone())
            .or_insert(FxHashSet::default())
            .insert(connection[0].clone());
    }
    let mut connected_3 = FxHashSet::default();
    for (t_starts, connected) in connections.iter() {
        if t_starts.starts_with("t") {
            let mut already_done = FxHashSet::default();
            for connection in connected.iter() {
                let next_connection = connections.get(connection).unwrap();
                for d in next_connection.intersection(connected) {
                    if !already_done.contains(d) {
                        let mut comp_3 = vec![t_starts, connection, d];
                        comp_3.sort_unstable();
                        connected_3.insert(comp_3);
                    }
                }
                already_done.insert(connection);
            }
        }
    }
    connected_3.len() as u64
}

use std::cmp::Reverse;

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let x = parse(input);
    let mut connections = FxHashMap::default();
    for connection in x {
        connections
            .entry(connection[0].clone())
            .or_insert(FxHashSet::default())
            .insert(connection[1].clone());
        connections
            .entry(connection[1].clone())
            .or_insert(FxHashSet::default())
            .insert(connection[0].clone());
    }
    let mut largest_so_far = FxHashSet::default();
    let mut checked_already = FxHashSet::default();
    for (start, connected) in connections.iter() {
        let mut trial = FxHashSet::default();
        let mut fully_connected = connected.clone();
        trial.insert(start);
        let mut connected_sorted = connected.iter().collect::<Vec<&String>>();
        connected_sorted.sort_unstable_by_key(|&x| Reverse(connections.get(x).unwrap().len()));

        for t in connected_sorted {
            if fully_connected.contains(t) {
                trial.insert(t);
                fully_connected = FxHashSet::from_iter(
                    fully_connected
                        .intersection(connections.get(t).unwrap())
                        .cloned(),
                );
            }
        }

        if trial.len() > largest_so_far.len() {
            largest_so_far = trial;
        }
        checked_already.insert(start);
    }
    println!("{:?}", largest_so_far.len());
    let mut output: Vec<_> = largest_so_far.into_iter().cloned().collect();
    output.sort_unstable();

    output.join(",")
}

use fixedbitset::{self, FixedBitSet};

fn string_to_int(s: &str) -> usize {
    assert!(s.len() == 2);
    let v = s.chars().collect::<Vec<char>>();
    (v[0] as u8 - b'a') as usize * 26 + (v[1] as u8 - b'a') as usize
}

fn int_to_string(mut s: usize) -> String {
    let mut output = "".to_string();
    for _ in 0..2 {
        let c = ((s % 26) as u8 + b'a') as char;
        output.push(c);
        s /= 26;
    }

    output.chars().rev().collect()
}

#[aoc(day23, part2, bronkerbosch)]
fn part2_bronkerbosch(input: &str) -> String {
    let x = parse(input);
    let mut connections = FxHashMap::default();
    for connection in x {
        connections
            .entry(string_to_int(&connection[0]))
            .or_insert(FixedBitSet::with_capacity(26 * 26))
            .insert(string_to_int(&connection[1]));
        connections
            .entry(string_to_int(&connection[1]))
            .or_insert(FixedBitSet::with_capacity(26 * 26))
            .insert(string_to_int(&connection[0]));
    }
    let mut cliques = FixedBitSet::with_capacity(26 * 26);
    let r = FixedBitSet::with_capacity(26 * 26);
    let p = FixedBitSet::from_iter(connections.keys().cloned());
    bron_kerbosch(r, p, &connections, &mut cliques);

    let mut output_string = cliques
        .into_ones()
        .map(int_to_string)
        .collect::<Vec<String>>();

    output_string.sort_unstable();

    output_string.join(",")
}

fn bron_kerbosch(
    r: FixedBitSet,
    mut p: FixedBitSet,
    graph: &FxHashMap<usize, FixedBitSet>,
    cliques: &mut FixedBitSet,
) {
    if p.is_clear() {
        if r.count_ones(..) > cliques.count_ones(..) {
            r.clone_into(cliques);
        }
    } else {
        if r.count_ones(..) + p.count_ones(..) < cliques.count_ones(..) {
            return;
        }
        let mut s_vec = p.ones().collect::<Vec<usize>>();
        s_vec.sort_by_key(|&x| Reverse(graph.get(&x).unwrap().len()));
        let pivot = s_vec.first().unwrap();
        let pivot_nbhd = graph.get(pivot).unwrap();
        for v in p.clone().ones() {
            if pivot_nbhd.contains(v) {
                continue;
            }
            let neighbors = graph.get(&v).unwrap();
            let mut new_node = FixedBitSet::with_capacity(26 * 26);
            new_node.insert(v);
            let mut r_new = r.clone();
            let mut p_new = p.clone();
            p_new.intersect_with(neighbors);
            r_new.union_with(&new_node);
            bron_kerbosch(r_new, p_new, graph, cliques);
            p.remove(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{file, fs, path::Path};

    #[test]
    fn test_1() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 7);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), "co,de,ka,ta".to_string());
    }

    #[test]
    fn test_string_conv() {
        let s = vec![
            "ab".to_string(),
            "cd".to_string(),
            "zz".to_string(),
            "im".to_string(),
        ];
        for p in s {
            assert_eq!(int_to_string(string_to_int(&p)), p);
        }
    }
}
