use rustc_hash::FxHashMap;

#[aoc(day1, part1)]
pub fn part_a(contents: &str) -> i32 {
    let (mut vec1, mut vec2) = parse(contents);
    vec1.sort_unstable();
    vec2.sort_unstable();
    vec1.into_iter()
        .zip(vec2)
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}

#[aoc(day1, part2)]
pub fn part_b(contents: &str) -> u64 {
    let (vec1, vec2) = parse(contents);
    let mut counter: FxHashMap<i32, u32> = FxHashMap::default();
    for val2 in vec2.into_iter() {
        *counter.entry(val2).or_default() += 1;
    }
    vec1.iter()
        .map(|x| *counter.get(x).unwrap_or(&0) as u64 * *x as u64)
        .sum()
}

pub fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let vec: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let vec1 = vec.iter().step_by(2).cloned().collect();
    let vec2 = vec.iter().skip(1).step_by(2).cloned().collect();
    (vec1, vec2)
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
        assert_eq!(part_a(&contents), 11);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 31);
    }
}
