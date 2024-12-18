use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<u64>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use fxhash::FxHashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let dir = [(0,1),(1,0),(0,usize::MAX), (usize::MAX,0)];
    let nr_bytes = match x.len() {
        25 => 12,
        3450 => 1024,
        _ => panic!()
    };
    let mut corrupted = FxHashSet::default();
    for idx in 0..nr_bytes {
        corrupted.insert((x[idx][0] as usize, x[idx][1] as usize));
    }
    let end = match x.len() {
        25 => (6,6),
        3450 => (70,70),
        _ => panic!()
    };
    let mut visited = FxHashSet::default();
    let mut p_q = BinaryHeap::new();
    p_q.push((Reverse(0), (0 as usize,0 as usize)));
    while let Some((dist, next_pos)) = p_q.pop() {
        if visited.contains(&next_pos) {
            continue;
        }
        if (next_pos) == end {
            return dist.0;
        }
        visited.insert(next_pos);
        for d in &dir {
            let trial = (next_pos.0.wrapping_add(d.0), next_pos.1.wrapping_add(d.1));
            if trial.0 <= end.0 && trial.1 <= end.1 {
                if !corrupted.contains(&trial) {
                    p_q.push((Reverse(dist.0 + 1), trial ));
                }
            }
        }
    } 

    return 0
}

#[aoc(day18, part2)]
fn part2(input: &str) -> String {
    let x = parse(input);
    let dir = [(0,1),(1,0),(0,usize::MAX), (usize::MAX,0)];
    let end = match x.len() {
        25 => (6,6),
        3450 => (70,70),
        _ => panic!()
    };
    for nr_bytes in 1..x.len() {
        let mut corrupted = FxHashSet::default();
        for idx in 0..nr_bytes {
            corrupted.insert((x[idx][0] as usize, x[idx][1] as usize));
        }
        let mut visited = FxHashSet::default();
        let mut p_q = BinaryHeap::new();
        p_q.push((Reverse(0), (0 as usize,0 as usize)));
        let mut exit_found = false;
        while let Some((dist, next_pos)) = p_q.pop() {
            if visited.contains(&next_pos) {
                continue;
            }
            if (next_pos) == end {
                exit_found = true;
                break;
            }
            visited.insert(next_pos);
            for d in &dir {
                let trial = (next_pos.0.wrapping_add(d.0), next_pos.1.wrapping_add(d.1));
                if trial.0 <= end.0 && trial.1 <= end.1 {
                    if !corrupted.contains(&trial) {
                        p_q.push((Reverse(dist.0 + 1), trial ));
                    }
                }
            }
        }
        if !exit_found {
            return format!("({},{})", x[nr_bytes-1][0],x[nr_bytes-1][1])
        }
    }

    return "(0,0)".to_string()
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
        assert_eq!(part1(&contents), 22);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), "(6,1)".to_string());
    }
}