fn parse(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    return output1;
}

use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0, 0);
    let mut guard_been = HashSet::new();
    let mut obstacles = HashSet::new();
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == '#' {
                obstacles.insert((x, y));
            }
            if vec1[x][y] == '^' {
                guard_pos = (x, y);
                guard_been.insert(guard_pos);
            }
        }
    }
    let dirs = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    loop {
        let mut next_pos = (
            guard_pos.0.wrapping_add(cur_dir.0),
            guard_pos.1.wrapping_add(cur_dir.1),
        );
        if obstacles.contains(&next_pos) {
            cur_dir_idx = (cur_dir_idx + 1) % 4;
            cur_dir = dirs[cur_dir_idx];
            next_pos = (
                guard_pos.0.wrapping_add(cur_dir.0),
                guard_pos.1.wrapping_add(cur_dir.1),
            )
        }
        if next_pos.0 >= vec1.len() || next_pos.1 >= vec1[0].len() {
            break;
        }
        guard_been.insert(next_pos);
        guard_pos = next_pos;
    }
    return guard_been.len() as i32;
}

fn check_loop(
    extra_obst: &(usize, usize),
    guard_orig_start: &(usize, usize),
    obstacle_per_row: &HashMap<usize, Vec<usize>>,
    obstacle_per_col: &HashMap<usize, Vec<usize>>,
    vec_size: &(usize, usize),
) -> bool {
    if extra_obst == guard_orig_start {
        return false;
    }
    let mut guard_been_new_dir = HashSet::new();
    let mut obst_row_new = obstacle_per_row.clone();
    let mut obst_col_new = obstacle_per_col.clone();
    let new_row_loc = obst_row_new
        .entry(extra_obst.0)
        .or_insert(vec![])
        .binary_search(&extra_obst.1)
        .unwrap_or_else(|e| e);
    obst_row_new
        .entry(extra_obst.0)
        .or_insert(vec![])
        .insert(new_row_loc, extra_obst.1);
    let new_col_loc = obst_col_new
        .entry(extra_obst.1)
        .or_insert(vec![])
        .binary_search(&extra_obst.0)
        .unwrap_or_else(|e| e);
    obst_col_new
        .entry(extra_obst.1)
        .or_insert(vec![])
        .insert(new_col_loc, extra_obst.0);

    let mut guard_pos = guard_orig_start.clone();
    let mut cur_dir_idx = 0;
    guard_been_new_dir.insert((guard_pos, cur_dir_idx));

    loop {
        let next_pos = match cur_dir_idx {
            0 => {
                let check = obst_col_new.get(&guard_pos.1);
                let next_obst_row_idx = check
                    .unwrap_or(&vec![])
                    .binary_search(&guard_pos.0)
                    .unwrap_or_else(|e| e);
                let next_obst_row = *check
                    .unwrap_or(&vec![])
                    .get(next_obst_row_idx - 1)
                    .unwrap_or(&(usize::MAX - 1));
                (next_obst_row.wrapping_add(1), guard_pos.1)
            }
            1 => {
                let check = obst_row_new.get(&guard_pos.0);
                let next_obst_col_idx = check
                    .unwrap_or(&vec![])
                    .binary_search(&guard_pos.1)
                    .unwrap_or_else(|e| e);
                let next_obst_col = *check
                    .unwrap_or(&vec![])
                    .get(next_obst_col_idx)
                    .unwrap_or(&(vec_size.1 + 1));
                (guard_pos.0, next_obst_col - 1)
            }
            2 => {
                let check = obst_col_new.get(&guard_pos.1);
                let next_obst_row_idx = check
                    .unwrap_or(&vec![])
                    .binary_search(&guard_pos.0)
                    .unwrap_or_else(|e| e);
                let next_obst_row = *check
                    .unwrap_or(&vec![])
                    .get(next_obst_row_idx)
                    .unwrap_or(&(vec_size.0 + 1));
                (next_obst_row - 1, guard_pos.1)
            }
            3 => {
                let check = obst_row_new.get(&guard_pos.0);
                let next_obst_col_idx = check
                    .unwrap_or(&vec![])
                    .binary_search(&guard_pos.1)
                    .unwrap_or_else(|e| e);
                let next_obst_col = *check
                    .unwrap_or(&vec![])
                    .get(next_obst_col_idx - 1)
                    .unwrap_or(&(usize::MAX - 1));
                (guard_pos.0, next_obst_col.wrapping_add(1))
            }
            _ => {
                unreachable!()
            }
        };

        cur_dir_idx = (cur_dir_idx + 1) % 4;
        if next_pos.0 >= vec_size.0 || next_pos.1 >= vec_size.1 {
            return false;
        }
        if guard_been_new_dir.contains(&(next_pos, cur_dir_idx)) {
            return true;
        }
        guard_been_new_dir.insert((next_pos, cur_dir_idx));
        guard_pos = next_pos;
    }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0, 0);

    let mut orig_obstacles = HashSet::new();
    let mut obstacle_per_row = HashMap::new();
    let mut obstacle_per_col = HashMap::new();
    let vec_size = (vec1.len(), vec1[0].len());
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == '#' {
                orig_obstacles.insert((x, y));
                obstacle_per_row.entry(x).or_insert(vec![]).push(y);
                obstacle_per_col.entry(y).or_insert(vec![]).push(x);
            }
            if vec1[x][y] == '^' {
                guard_pos = (x, y);
            }
        }
    }
    obstacle_per_col
        .values_mut()
        .for_each(|x| x.sort_unstable());
    obstacle_per_row
        .values_mut()
        .for_each(|x| x.sort_unstable());
    let guard_orig_start = guard_pos.clone();
    let mut guard_been = HashSet::new();
    guard_been.insert(guard_pos);
    let dirs = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;

    loop {
        let mut next_pos = (
            guard_pos.0.wrapping_add(cur_dir.0),
            guard_pos.1.wrapping_add(cur_dir.1),
        );
        if orig_obstacles.contains(&next_pos) {
            cur_dir_idx = (cur_dir_idx + 1) % 4;
            cur_dir = dirs[cur_dir_idx];
            next_pos = (
                guard_pos.0.wrapping_add(cur_dir.0),
                guard_pos.1.wrapping_add(cur_dir.1),
            )
        }
        if next_pos.0 >= vec_size.0 || next_pos.1 >= vec_size.1 {
            break;
        }
        guard_been.insert(next_pos);
        guard_pos = next_pos;
    }
    let nr_loops = guard_been
        .par_iter()
        .filter(|&x| {
            check_loop(
                x,
                &guard_orig_start,
                &obstacle_per_row,
                &obstacle_per_col,
                &vec_size,
            )
        })
        .count() as i32;
    return nr_loops;
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
        assert_eq!(part1(&contents), 41);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 6);
    }
}
