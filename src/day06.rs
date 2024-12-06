fn parse(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    return output1;
}

use fxhash::FxHashSet;

use rayon::prelude::*;

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
    return guard_been.len() as i32;
}

fn find_next_larger(sorted_vec: &Vec<usize>, val: &usize, default: usize) -> usize {
    let larger_idx = sorted_vec.binary_search(val).unwrap_or_else(|e| e);
    let larger_val = *sorted_vec.get(larger_idx).unwrap_or(&(default + 1));
    return larger_val;
}

fn find_next_smaller(sorted_vec: &Vec<usize>, val: &usize, default: usize) -> usize {
    let smaller_idx = sorted_vec.binary_search(val).unwrap_or_else(|e| e);
    let smaller_val = *sorted_vec.get(smaller_idx.wrapping_sub(1)).unwrap_or(&default);
    return smaller_val;
}

fn insert_sorted(sorted_vec: &mut Vec<usize>, val: usize) {
    let insert_idx = sorted_vec.binary_search(&val).unwrap_or_else(|e| e);
    sorted_vec.insert(insert_idx, val);
}

fn check_loop(
    extra_obst: &(usize, usize),
    guard_orig_start: &(usize, usize),
    obstacle_per_row: &Vec<Vec<usize>>,
    obstacle_per_col: &Vec<Vec<usize>>,
    vec_size: &(usize, usize),
) -> bool {
    if extra_obst == guard_orig_start {
        return false;
    }
    // println!("Checking {:?}", extra_obst);
    let mut guard_been_new_dir = FxHashSet::default();
    let mut obst_row_new = obstacle_per_row.clone();
    let mut obst_col_new = obstacle_per_col.clone();
    insert_sorted(
        &mut obst_row_new[extra_obst.0],
        extra_obst.1,
    );
    insert_sorted(
        &mut obst_col_new[extra_obst.1],
        extra_obst.0,
    );

    let mut guard_pos = guard_orig_start.clone();
    let mut cur_dir_idx = 0;
    guard_been_new_dir.insert((guard_pos, cur_dir_idx));

    loop {
        let next_pos = match cur_dir_idx {
            0 => {
                let check = &obst_col_new[guard_pos.1];
                let next_obst_row =
                    find_next_smaller(check, &guard_pos.0, usize::MAX - 1);
                (next_obst_row + 1, guard_pos.1)
            }
            1 => {
                let check = &obst_row_new[guard_pos.0];
                let next_obst_col =
                    find_next_larger(check, &guard_pos.1, vec_size.1);
                (guard_pos.0, next_obst_col - 1)
            }
            2 => {
                let check = &obst_col_new[guard_pos.1];
                let next_obst_row =
                    find_next_larger(check, &guard_pos.0, vec_size.0);
                (next_obst_row - 1, guard_pos.1)
            }
            3 => {
                let check = &obst_row_new[guard_pos.0];
                let next_obst_col =
                    find_next_smaller(check, &guard_pos.1, usize::MAX - 1);
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
        if guard_been_new_dir.contains(&(next_pos, cur_dir_idx)) {
            // println!("Loop fount at {:?}",extra_obst);
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

    let mut orig_obstacles = FxHashSet::default();
    // let mut obstacle_per_row = FxHashMap::default();
    // let mut obstacle_per_col = FxHashMap::default();
    let vec_size = (vec1.len(), vec1[0].len());
    let mut obstacle_per_row = vec![vec![];vec_size.0];
    let mut obstacle_per_col = vec![vec![];vec_size.1];
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == '#' {
                orig_obstacles.insert((x, y));
                obstacle_per_row[x].push(y);
                obstacle_per_col[y].push(x);
            }
            if vec1[x][y] == '^' {
                guard_pos = (x, y);
            }
        }
    }
    
    // println!("{:?}",obstacle_per_col.values().map(|x| x.len()).max().unwrap() );
    // println!("{:?}",obstacle_per_row.values().map(|x| x.len()).max().unwrap() );
    for subvec in obstacle_per_col.iter_mut() {
        subvec.sort_unstable();
    }
    for subvec in obstacle_per_row.iter_mut() {
        subvec.sort_unstable();
    }
    let guard_orig_start = guard_pos.clone();
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
