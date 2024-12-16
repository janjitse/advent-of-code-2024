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
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    loc: (usize, usize),
    facing: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
struct PriorityElement {
    dist: i32,
    pos: Position,
    hist: Option<Position>,
}

impl Ord for PriorityElement {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.dist).cmp(&Reverse(other.dist))
    }
}

impl PartialOrd for PriorityElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Reverse(self.dist).cmp(&Reverse(other.dist)))
    }
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
    let mut p_q = BinaryHeap::new();
    let mut visited = FxHashSet::default();
    p_q.push(PriorityElement {
        dist: 0,
        pos: starting_pos,
        hist: None,
    });
    while let Some(PriorityElement {
        dist: d,
        pos,
        hist: _,
    }) = p_q.pop()
    {
        if pos.loc == end {
            return d as u64;
        }
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos.clone());
        let mut cur_dir = pos.facing;
        for rot_cost in 0..4 {
            if rot_cost == 2 {
                cur_dir = clockwise[&cur_dir];
                continue;
            }
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                p_q.push(PriorityElement {
                    dist: d + (rot_cost % 2) * 1000 + 1,
                    pos: xy,
                    hist: None,
                });
            }
            cur_dir = clockwise[&cur_dir];
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
            } else if col == 'S' {
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
    let mut p_q = BinaryHeap::new();
    let mut visited_distance = FxHashMap::default();
    let mut previous_elements: FxHashMap<Position, FxHashSet<Option<Position>>> =
        FxHashMap::default();
    let previous = None;
    p_q.push(PriorityElement {
        dist: 0,
        pos: starting_pos,
        hist: previous,
    });
    let mut end_distance = i32::MAX;
    while let Some(PriorityElement {
        dist: d,
        pos,
        hist: prev,
    }) = p_q.pop()
    {
        if d > end_distance {
            break;
        }
        if d > *visited_distance.get(&pos).unwrap_or(&i32::MAX) {
            continue;
        }
        if pos.loc == end {
            end_distance = d;
            previous_elements.entry(pos).or_default().insert(prev);
            continue;
        }
        visited_distance.insert(pos.clone(), d);
        previous_elements
            .entry(pos.clone())
            .or_default()
            .insert(prev.clone());
        let mut cur_dir = pos.facing;
        for rot_cost in 0..4 {
            if rot_cost == 2 {
                // don't go back
                cur_dir = clockwise[&cur_dir];
                continue;
            }
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                    facing: cur_dir,
                };
                p_q.push(PriorityElement {
                    dist: d + (rot_cost % 2) * 1000 + 1,
                    pos: xy,
                    hist: Some(pos.clone()),
                });
            }
            cur_dir = clockwise[&cur_dir];
        }
    }
    let mut locations = FxHashSet::default();
    locations.insert(end);
    let mut walkback = vec![];
    for &facing in clockwise.values() {
        let end_pos = Position { loc: end, facing };
        let end_elements = previous_elements.remove(&end_pos);
        if let Some(end_elements) = end_elements {
            walkback.extend(end_elements.into_iter());
        }
    }
    while let Some(pos) = walkback.pop() {
        if let Some(pos) = pos {
            locations.insert(pos.loc);
            let new_elements = previous_elements.remove(&pos);
            if let Some(new_elements) = new_elements {
                walkback.extend(new_elements.into_iter());
            }
        }
    }
    locations.len() as u64
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
