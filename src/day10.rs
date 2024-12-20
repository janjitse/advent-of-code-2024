use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<u32>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let x = parse(input);
    let mut trailheads = vec![];
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 0 {
                trailheads.push((row_idx, col_idx));
            }
        }
    }
    let directions = [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)];
    let mut output = 0;
    for start in trailheads {
        let mut visited = FxHashSet::default();
        let mut todo = VecDeque::new();
        todo.push_back((start, 0));
        while let Some((next_node, height)) = todo.pop_front() {
            for dir in directions {
                if next_node.0.wrapping_add(dir.0) < x.len()
                    && next_node.1.wrapping_add(dir.1) < x[0].len()
                {
                    let cand = (
                        next_node.0.wrapping_add(dir.0),
                        next_node.1.wrapping_add(dir.1),
                    );
                    if x[cand.0][cand.1] == height + 1 {
                        if height + 1 == 9 {
                            visited.insert(cand);
                        } else {
                            todo.push_back((cand, height + 1));
                        }
                    }
                }
            }
        }
        // println!("trailhead {:?}, score {:?}", start, visited.len() );
        output += visited.len() as i32;
    }
    output
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut trailheads = vec![];
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 0 {
                trailheads.push((row_idx, col_idx));
            }
        }
    }
    let directions = [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)];
    let mut output = 0;
    for start in trailheads {
        let mut todo = VecDeque::new();
        todo.push_back((start, 0));
        while let Some((next_node, height)) = todo.pop_front() {
            for dir in directions {
                if next_node.0.wrapping_add(dir.0) < x.len()
                    && next_node.1.wrapping_add(dir.1) < x[0].len()
                {
                    let cand = (
                        next_node.0.wrapping_add(dir.0),
                        next_node.1.wrapping_add(dir.1),
                    );
                    if x[cand.0][cand.1] == height + 1 {
                        if height + 1 == 9 {
                            output += 1;
                        } else {
                            todo.push_back((cand, height + 1));
                        }
                    }
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{file, fs, path::Path};

    #[test]
    fn test_1() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 36);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 81);
    }
}
