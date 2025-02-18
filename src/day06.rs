fn parse(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

use fixedbitset::FixedBitSet;
use rustc_hash::FxHashSet;

use rayon::prelude::*;

#[aoc(day6, part1, bitset)]
fn part1_bitset(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0, 0);
    let width = vec1[0].len();
    let height = vec1.len();
    let mut guard_been = FixedBitSet::with_capacity(width * height);
    let mut obstacles = FixedBitSet::with_capacity(width * height);
    for x in 0..height {
        for y in 0..width {
            if vec1[x][y] == '#' {
                obstacles.insert(x * width + y);
            }
            if vec1[x][y] == '^' {
                guard_pos = (x, y);
            }
        }
    }
    guard_been.insert(guard_pos.0 * width + guard_pos.1);
    let dirs = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    loop {
        let mut next_pos = (
            guard_pos.0.wrapping_add(cur_dir.0),
            guard_pos.1.wrapping_add(cur_dir.1),
        );
        while next_pos.0 < height
            && next_pos.1 < width
            && obstacles.contains(next_pos.0 * width + next_pos.1)
        {
            cur_dir_idx = (cur_dir_idx + 1) % 4;
            cur_dir = dirs[cur_dir_idx];
            next_pos = (
                guard_pos.0.wrapping_add(cur_dir.0),
                guard_pos.1.wrapping_add(cur_dir.1),
            )
        }
        if next_pos.0 >= height || next_pos.1 >= width {
            break;
        }
        guard_been.insert(next_pos.0 * width + next_pos.1);
        guard_pos = next_pos;
    }
    guard_been.count_ones(..) as i32
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0, 0);
    let mut guard_been = FxHashSet::default();
    let mut obstacles = FxHashSet::default();
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == '#' {
                obstacles.insert((x, y));
            }
            if vec1[x][y] == '^' {
                guard_pos = (x, y);
            }
        }
    }
    guard_been.insert(guard_pos);
    let dirs = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    loop {
        let mut next_pos = (
            guard_pos.0.wrapping_add(cur_dir.0),
            guard_pos.1.wrapping_add(cur_dir.1),
        );
        while obstacles.contains(&next_pos) {
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
    guard_been.len() as i32
}

fn find_next_larger(sorted_vec: &[usize], val: usize, default: usize) -> usize {
    let larger_idx = sorted_vec.binary_search(&val).unwrap_or_else(|e| e);
    *sorted_vec.get(larger_idx).unwrap_or(&(default + 1))
}

fn find_next_smaller(sorted_vec: &[usize], val: usize) -> usize {
    let smaller_idx = sorted_vec.binary_search(&val).unwrap_or_else(|e| e);
    *sorted_vec
        .get(smaller_idx.wrapping_sub(1))
        .unwrap_or(&(usize::MAX - 1))
}

fn insert_sorted(sorted_vec: &mut Vec<usize>, val: usize) {
    let insert_idx = sorted_vec.binary_search(&val).unwrap_or_else(|e| e);
    sorted_vec.insert(insert_idx, val);
}

fn check_loop(
    extra_obst: &(usize, usize),
    guard_orig_start: &(usize, usize),
    obstacle_per_row: &[Vec<usize>],
    obstacle_per_col: &[Vec<usize>],
    vec_size: &(usize, usize),
) -> bool {
    if extra_obst == guard_orig_start {
        return false;
    }
    let mut guard_been_new_dir = FixedBitSet::with_capacity(vec_size.0 * vec_size.1 * 4);
    let mut obst_row_new = obstacle_per_row.to_owned();
    let mut obst_col_new = obstacle_per_col.to_owned();
    insert_sorted(&mut obst_row_new[extra_obst.0], extra_obst.1);
    insert_sorted(&mut obst_col_new[extra_obst.1], extra_obst.0);

    let mut guard_pos = *guard_orig_start;
    let mut cur_dir_idx = 0;
    guard_been_new_dir.insert(guard_pos.0 * vec_size.0 * 4 + guard_pos.1 * 4 + cur_dir_idx);

    loop {
        let next_pos = match cur_dir_idx {
            0 => {
                let next_obst_row = find_next_smaller(&obst_col_new[guard_pos.1], guard_pos.0);
                (next_obst_row + 1, guard_pos.1)
            }
            1 => {
                let next_obst_col =
                    find_next_larger(&obst_row_new[guard_pos.0], guard_pos.1, vec_size.1);
                (guard_pos.0, next_obst_col - 1)
            }
            2 => {
                let next_obst_row =
                    find_next_larger(&obst_col_new[guard_pos.1], guard_pos.0, vec_size.0);
                (next_obst_row - 1, guard_pos.1)
            }
            3 => {
                let next_obst_col = find_next_smaller(&obst_row_new[guard_pos.0], guard_pos.1);
                (guard_pos.0, next_obst_col + 1)
            }
            _ => {
                unreachable!()
            }
        };

        if next_pos.0 >= vec_size.0 || next_pos.1 >= vec_size.1 {
            return false;
        }
        cur_dir_idx = (cur_dir_idx + 1) % 4;
        if next_pos.0 < vec_size.0
            && next_pos.1 < vec_size.1
            && guard_been_new_dir
                .contains(next_pos.0 * vec_size.0 * 4 + next_pos.1 * 4 + cur_dir_idx)
        {
            return true;
        }

        guard_been_new_dir.insert(next_pos.0 * vec_size.0 * 4 + next_pos.1 * 4 + cur_dir_idx);
        guard_pos = next_pos;
    }
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0, 0);

    let mut orig_obstacles = FxHashSet::default();
    let vec_size = (vec1.len(), vec1[0].len());
    let mut obstacle_per_row = vec![vec![]; vec_size.0];
    let mut obstacle_per_col = vec![vec![]; vec_size.1];
    for x in 0..vec1.len() {
        for (y, &c) in vec1[x].iter().enumerate() {
            if c == '#' {
                orig_obstacles.insert((x, y));
                obstacle_per_row[x].push(y);
                obstacle_per_col[y].push(x);
            } else if c == '^' {
                guard_pos = (x, y);
            }
        }
    }

    for subvec in obstacle_per_col.iter_mut() {
        subvec.sort_unstable();
    }
    for subvec in obstacle_per_row.iter_mut() {
        subvec.sort_unstable();
    }
    let guard_orig_start = guard_pos;
    let mut guard_been = FxHashSet::default();
    let dirs = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    guard_been.insert(guard_pos);
    loop {
        let mut next_pos = (
            guard_pos.0.wrapping_add(cur_dir.0),
            guard_pos.1.wrapping_add(cur_dir.1),
        );
        while orig_obstacles.contains(&next_pos) {
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
    guard_been
        .into_par_iter()
        .filter(|&x| {
            check_loop(
                &x,
                &guard_orig_start,
                &obstacle_per_row,
                &obstacle_per_col,
                &vec_size,
            )
        })
        .count() as i32
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
