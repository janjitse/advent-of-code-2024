use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<char>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Position {
    loc: (usize, usize),
    facing: (usize, usize),
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut spaces = FxHashSet::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '.' || col == 'S' || col == 'E' {
                spaces.insert((row_idx, col_idx));
            }
            if col == 'E' {
                end = (row_idx, col_idx);
            }
            if col == 'S' {
                start = (row_idx, col_idx);
            }
        }
    }
    let starting_pos = Position {
        loc: start,
        facing: (0, 1),
    };
    let clockwise = FxHashMap::from_iter([
        ((0, 1), (1, 0)),
        ((1, 0), (0, usize::MAX)),
        ((0, usize::MAX), (usize::MAX, 0)),
        ((usize::MAX, 0), (0, 1)),
    ]);
    let counter_clockwise = FxHashMap::from_iter([
        ((1, 0), (0, 1)),
        ((0, usize::MAX), (1, 0)),
        ((usize::MAX, 0), (0, usize::MAX)),
        ((0, 1), (usize::MAX, 0)),
    ]);
    let mut p_q = BinaryHeap::new();
    let mut visited = FxHashSet::default();
    p_q.push((Reverse(0), starting_pos));
    while let Some((d, pos)) = p_q.pop() {
        if pos.loc == end {
            return d.0;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos.clone());
        let mut cur_dir = pos.facing;
        for rot_cost in 0..3 {
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                p_q.push((Reverse(d.0 + rot_cost * 1000 + 1), xy));
            }
            cur_dir = clockwise[&cur_dir];
        }
        cur_dir = pos.facing;
        for rot_cost in 1..2 {
            cur_dir = counter_clockwise[&cur_dir];
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                p_q.push((Reverse(d.0 + rot_cost * 1000 + 1), xy));
            }
        }
    }
    0
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut spaces = FxHashSet::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '.' || col == 'S' || col == 'E' {
                spaces.insert((row_idx, col_idx));
            }
            if col == 'E' {
                end = (row_idx, col_idx);
            }
            if col == 'S' {
                start = (row_idx, col_idx);
            }
        }
    }
    let starting_pos = Position {
        loc: start,
        facing: (0, 1),
    };
    let clockwise = FxHashMap::from_iter([
        ((0, 1), (1, 0)),
        ((1, 0), (0, usize::MAX)),
        ((0, usize::MAX), (usize::MAX, 0)),
        ((usize::MAX, 0), (0, 1)),
    ]);
    let counter_clockwise = FxHashMap::from_iter([
        ((1, 0), (0, 1)),
        ((0, usize::MAX), (1, 0)),
        ((usize::MAX, 0), (0, usize::MAX)),
        ((0, 1), (usize::MAX, 0)),
    ]);
    let mut p_q = BinaryHeap::new();
    let mut visited_distance = FxHashMap::default();
    let previous: Vec<(usize, usize)> = vec![];
    p_q.push((Reverse(0), starting_pos, previous));
    let mut all_prev = vec![];
    let mut end_distance = i32::MAX;
    while let Some((d, pos, prev)) = p_q.pop() {
        if d.0 > end_distance {
            break;
        }
        if pos.loc == end {
            // println!("Path to end found: {:?}", d.0);
            end_distance = d.0;
            all_prev.push(prev.clone());
            continue;
        }
        if visited_distance.contains_key(&pos) && d.0 > *visited_distance.get(&pos).unwrap() {
            continue;
        }
        visited_distance.insert(pos.clone(), d.0);
        let mut cur_dir = pos.facing;
        for rot_cost in 0..2 {
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                let mut next_prev = prev.clone();
                next_prev.push(pos.loc);
                p_q.push((Reverse(d.0 + rot_cost * 1000 + 1), xy, next_prev));
            }
            cur_dir = clockwise[&cur_dir];
        }
        cur_dir = pos.facing;
        for rot_cost in 1..2 {
            cur_dir = counter_clockwise[&cur_dir];
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                let mut next_prev = prev.clone();
                next_prev.push(pos.loc);
                p_q.push((Reverse(d.0 + rot_cost * 1000 + 1), xy, next_prev));
            }
        }
    }
    let mut locations = FxHashSet::default();
    for path in all_prev {
        for p in path {
            locations.insert(p);
        }
    }
    locations.len() as u64 + 1
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
        assert_eq!(part1(&contents), 7036);
    }

    #[test]
    fn test_1b() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 11048);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 45);
    }

    #[test]
    fn test_2b() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 64);
    }
}
