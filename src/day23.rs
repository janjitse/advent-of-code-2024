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

    // println!("{:?}", connected_3);

    connected_3.len() as u64
}

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
    for (start, connected) in connections.iter() {
        if largest_so_far.contains(start) {
            continue;
        }
        let mut trial = FxHashSet::default();
        let mut fully_connected = connected.clone();
        trial.insert(start);
        for t in connected {
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
    }
    println!("{:?}", largest_so_far.len());
    let mut output: Vec<_> = largest_so_far.into_iter().cloned().collect();
    output.sort_unstable();
    output.join(",")
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
}
