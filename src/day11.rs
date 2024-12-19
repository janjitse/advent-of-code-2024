use std::time::SystemTime;

fn parse(input: &str) -> Vec<u64> {
    let time_start = SystemTime::now();
    let output1 = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use fxhash::FxHashMap;

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    let output = parse(input);
    let mut hash = FxHashMap::default();
    for d in output {
        *hash.entry(d).or_default() += 1;
    }
    for _ in 0..25 {
        let mut next_hash = FxHashMap::default();
        for (idx, amount) in hash {
            let length = idx.checked_ilog10().unwrap_or(0) + 1;
            if idx == 0 {
                *next_hash.entry(1).or_default() += amount;
            } else if length % 2 == 0 {
                let power10 = 10u64.pow(length / 2);
                *next_hash.entry(idx / power10).or_default() += amount;
                *next_hash.entry(idx % power10).or_default() += amount;
            } else {
                *next_hash.entry(idx * 2024).or_default() += amount;
            }
        }
        hash = next_hash;
    }
    hash.values().sum()
}

#[allow(dead_code)]
// #[aoc(day11, part1, rec)]
fn part1_rec(input: &str) -> u64 {
    let output = parse(input);
    let mut cache = FxHashMap::default();
    output
        .into_iter()
        .map(|d| recurse_mem(25, d, &mut cache))
        .sum()
}

fn recurse_mem(blinks_todo: u8, idx: u64, cache: &mut FxHashMap<(u8, u64), u64>) -> u64 {
    if blinks_todo == 0 {
        return 1;
    }
    if let Some(total) = cache.get(&(blinks_todo, idx)) {
        return *total;
    }
    let total;
    let length = idx.checked_ilog10().unwrap_or(0) + 1;
    if idx == 0 {
        total = recurse_mem(blinks_todo - 1, 1, cache);
    } else if length % 2 == 0 {
        let power10 = 10u64.pow(length / 2);
        total = recurse_mem(blinks_todo - 1, idx / power10, cache)
            + recurse_mem(blinks_todo - 1, idx % power10, cache);
    } else {
        total = recurse_mem(blinks_todo - 1, idx * 2024, cache);
    }
    cache.insert((blinks_todo, idx), total);
    total
}

type Counter = FxHashMap<u64, u64>;

#[allow(dead_code)]
fn recurse_dict(blinks_todo: u8, idx: u64, cache: &mut FxHashMap<(u8, u64), Counter>) -> Counter {
    if blinks_todo == 0 {
        return Counter::from_iter(vec![(idx, 1u64)]);
    }
    if let Some(total) = cache.get(&(blinks_todo, idx)) {
        return total.clone();
    }
    let mut total: Counter;
    let length = idx.checked_ilog10().unwrap_or(0) + 1;
    if idx == 0 {
        total = recurse_dict(blinks_todo - 1, 1, cache);
    } else if length % 2 == 0 {
        let power10 = 10u64.pow(length / 2);
        total = recurse_dict(blinks_todo - 1, idx / power10, cache);
        let total2 = recurse_dict(blinks_todo - 1, idx % power10, cache);
        total2
            .into_iter()
            .for_each(|(t_idx, t_amount)| *total.entry(t_idx).or_default() += t_amount);
    } else {
        total = recurse_dict(blinks_todo - 1, idx * 2024, cache);
    }
    let s = total.clone();
    cache.insert((blinks_todo, idx), s);
    total
}

#[allow(dead_code)]
// #[aoc(day11, part1, rec_dict)]
fn part1_rec_dict(input: &str) -> u64 {
    let output = parse(input);
    let mut total = Counter::default();
    let mut cache = FxHashMap::default();
    for d in output {
        *total.entry(d).or_default() += 1;
    }
    for _ in 0..5 {
        let mut new_hash = Counter::default();
        for (d, new_amount) in total {
            for (idx, amount) in recurse_dict(5, d, &mut cache) {
                *new_hash.entry(idx).or_default() += amount * new_amount;
            }
        }
        total = new_hash;
    }
    total.values().sum()
}

#[allow(dead_code)]
// #[aoc(day11, part2, rec_dict)]
fn part2_rec_dict(input: &str) -> u64 {
    let output = parse(input);
    let mut total = Counter::default();
    let mut cache = FxHashMap::default();
    for d in output {
        *total.entry(d).or_default() += 1;
    }

    for _ in 0..25 {
        let mut new_hash = Counter::default();
        for (d, orig_amount) in total {
            for (idx, amount) in recurse_dict(3, d, &mut cache) {
                *new_hash.entry(idx).or_default() += amount * orig_amount;
            }
        }
        total = new_hash;
    }

    total.values().sum()
}

#[allow(dead_code)]
// #[aoc(day11, part2, rec)]
fn part2_rec(input: &str) -> u64 {
    let output = parse(input);
    let mut cache = FxHashMap::default();
    output
        .into_iter()
        .map(|d| recurse_mem(75, d, &mut cache))
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u64 {
    let output = parse(input);
    let mut hash = FxHashMap::default();
    for d in output {
        *hash.entry(d).or_default() += 1;
    }
    for _ in 0..75 {
        let mut next_hash = FxHashMap::default();
        for (idx, amount) in hash {
            let length = idx.checked_ilog10().unwrap_or(0) + 1;
            if idx == 0 {
                *next_hash.entry(1).or_default() += amount;
            } else if length % 2 == 0 {
                let power10 = 10u64.pow(length / 2);
                *next_hash.entry(idx / power10).or_default() += amount;
                *next_hash.entry(idx % power10).or_default() += amount;
            } else {
                *next_hash.entry(idx * 2024).or_default() += amount;
            }
        }
        hash = next_hash;
    }
    hash.values().sum()
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
        assert_eq!(part1(&contents), 55312);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 65601038650482);
    }
}
