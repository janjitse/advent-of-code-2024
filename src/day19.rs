use std::time::SystemTime;

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(", ").map(|x| x.to_string()).collect())
        .next()
        .unwrap();
    let output2 = lines
        .by_ref()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (output1, output2)
}

use fxhash::FxHashMap;

#[aoc(day19, part1)]
fn part1(input: &str) -> u64 {
    let (towels, patterns) = parse(input);
    // println!("{:?}", towels);
    // println!("{:?}", patterns);
    let mut counter = 0;
    for p in patterns {
        let mut cache: FxHashMap<usize, bool> = FxHashMap::default();
        if recurse(&p, &towels, &mut cache) {
            counter += 1;
        }
    }
    counter
}

fn recurse(remaining: &str, towels: &[String], cache: &mut FxHashMap<usize, bool>) -> bool {
    if remaining.is_empty() {
        return true;
    }
    if cache.contains_key(&remaining.len()) {
        return cache[&remaining.len()];
    }
    let mut possible = false;
    for t in towels {
        if remaining.starts_with(t) {
            possible |= recurse(&remaining[t.len()..], towels, cache);
        }
        if possible {
            break;
        }
    }
    cache.insert(remaining.len(), possible);
    possible
}

fn recurse_count(remaining: &str, towels: &[String], cache: &mut Vec<i64>) -> i64 {
    if remaining.is_empty() {
        return 1;
    }
    if cache[remaining.len()] >= 0 {
        return cache[remaining.len()];
    }
    let mut possible = 0;
    for t in towels {
        if remaining.starts_with(t) {
            possible += recurse_count(&remaining[t.len()..], towels, cache);
        }
    }
    cache[remaining.len()] = possible;
    possible
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let (towels, patterns) = parse(input);
    let mut counter = 0;
    
    for p in patterns {
        let mut cache = vec![-1; p.len()+1];
        counter += recurse_count(&p, &towels, &mut cache);
    }
    counter
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
        assert_eq!(part1(&contents), 6);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 16);
    }
}
