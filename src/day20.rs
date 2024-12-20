use std::time::SystemTime;

fn parse(input: &str) -> (FxHashSet<(usize, usize)>, (usize, usize), (usize, usize)) {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut spaces = FxHashSet::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row_idx, row) in output1.iter().enumerate() {
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
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (spaces, start, end)
}

use fxhash::{FxHashMap, FxHashSet};

#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let (spaces, start, end) = parse(input);

    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut distance_ord = FxHashMap::default();
    let mut cur_loc = start;
    distance_ord.insert(start, 0);
    let mut cur_dist = 0;
    while cur_loc != end {
        for dir in directions.iter() {
            let new_loc = (cur_loc.0.wrapping_add(dir.0), cur_loc.1.wrapping_add(dir.1));
            if spaces.contains(&new_loc) && !distance_ord.contains_key(&new_loc) {
                cur_loc = new_loc;
                break;
            }
        }
        cur_dist += 1;
        distance_ord.insert(cur_loc, cur_dist);
    }

    let mut cheating_paths = 0;
    let full_distance = *distance_ord.get(&end).unwrap();
    for cheat_start_loc in spaces.iter() {
        let cheat_start_length = distance_ord[cheat_start_loc];
        for y_delta in -2..=2_isize {
            for x_delta in -2 + y_delta.abs()..=2_isize - y_delta.abs() {
                let cheat_end_loc = (
                    cheat_start_loc.0.wrapping_add(y_delta as usize),
                    cheat_start_loc.1.wrapping_add(x_delta as usize),
                );
                if let Some(&cheat_end_length) = distance_ord.get(&cheat_end_loc) {
                    let cheat_length = x_delta.abs() + y_delta.abs();
                    if cheat_start_length + (full_distance - cheat_end_length) + cheat_length as i64
                        <= full_distance - 100
                    {
                        cheating_paths += 1;
                    }
                }
            }
        }
    }
    cheating_paths
}

#[aoc(day20, part2)]
fn part2(input: &str) -> u64 {
    let (spaces, start, end) = parse(input);

    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut distance_ord = FxHashMap::default();
    let mut cur_loc = start;
    distance_ord.insert(start, 0);
    let mut cur_dist = 0;
    while cur_loc != end {
        for dir in directions.iter() {
            let new_loc = (cur_loc.0.wrapping_add(dir.0), cur_loc.1.wrapping_add(dir.1));
            if spaces.contains(&new_loc) && !distance_ord.contains_key(&new_loc) {
                cur_loc = new_loc;
                break;
            }
        }
        cur_dist += 1;
        distance_ord.insert(cur_loc, cur_dist);
    }

    let mut cheating_paths = 0;
    let full_distance = *distance_ord.get(&end).unwrap();
    for cheat_start_loc in spaces.iter() {
        let cheat_start_length = distance_ord[cheat_start_loc];
        for y_delta in -20..=20_isize {
            for x_delta in -20 + y_delta.abs()..=20_isize - y_delta.abs() {
                let cheat_end_loc = (
                    cheat_start_loc.0.wrapping_add(y_delta as usize),
                    cheat_start_loc.1.wrapping_add(x_delta as usize),
                );
                if let Some(&cheat_end_length) = distance_ord.get(&cheat_end_loc) {
                    let cheat_length = x_delta.abs() + y_delta.abs();
                    if cheat_start_length + (full_distance - cheat_end_length) + cheat_length as i64
                        <= full_distance - 100
                    {
                        cheating_paths += 1;
                    }
                }
            }
        }
    }
    cheating_paths
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
        assert_eq!(part1(&contents), 0);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 0);
    }
}
