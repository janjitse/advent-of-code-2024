use std::time::SystemTime;

fn parse(
    input: &str,
) -> (
    FxHashSet<(usize, usize)>,
    (usize, usize),
    (usize, usize),
    Vec<(usize, usize)>,
) {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut spaces = FxHashSet::default();
    let mut spaces_vec = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row_idx, row) in output1.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '.' || col == 'S' || col == 'E' {
                spaces.insert((row_idx, col_idx));
                spaces_vec.push((row_idx, col_idx));
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
    (spaces, start, end, spaces_vec)
}

use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let (spaces, start, end, spaces_vec) = parse(input);
    solve(spaces, start, end, spaces_vec, 2)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> u64 {
    let (spaces, start, end, spaces_vec) = parse(input);
    solve(spaces, start, end, spaces_vec, 20)
}

fn solve(
    spaces: FxHashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    spaces_vec: Vec<(usize, usize)>,
    max_cheat: isize,
) -> u64 {
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let max_height = spaces_vec.iter().max_by_key(|x| x.0).unwrap().0 + 1;
    let max_width = spaces_vec.iter().max_by_key(|x| x.1).unwrap().1 + 1;
    let mut distance_ord = vec![vec![u64::MAX; max_width]; max_height];
    let mut cur_loc = start;
    distance_ord[start.0][start.1] = 0;
    let mut cur_dist = 0;
    while cur_loc != end {
        for dir in directions.iter() {
            let new_loc = (cur_loc.0.wrapping_add(dir.0), cur_loc.1.wrapping_add(dir.1));
            if spaces.contains(&new_loc) && distance_ord[new_loc.0][new_loc.1] == u64::MAX {
                cur_loc = new_loc;
                break;
            }
        }
        cur_dist += 1;
        distance_ord[cur_loc.0][cur_loc.1] = cur_dist;
    }
    spaces_vec
        .into_par_iter()
        .map(|cheat_start_loc| {
            (0..=max_cheat.min((max_height - cheat_start_loc.0 - 1) as isize))
                .map(|y_delta| {
                    let distance_ord = &distance_ord;
                    let range = if y_delta > 0 {
                        (-max_cheat + y_delta.abs()).max(-(cheat_start_loc.1 as isize))
                            ..=(max_cheat - y_delta.abs())
                                .min((max_width - cheat_start_loc.1 - 1) as isize)
                    } else {
                        2..=(max_cheat.min((max_width - cheat_start_loc.1 - 1) as isize))
                    };
                    range
                        .filter(move |x_delta| {
                            let cheat_end_loc = (
                                cheat_start_loc.0.wrapping_add(y_delta as usize),
                                cheat_start_loc.1.wrapping_add(*x_delta as usize),
                            );
                            if distance_ord[cheat_end_loc.0][cheat_end_loc.1] < u64::MAX {
                                let cheat_start_length =
                                    distance_ord[cheat_start_loc.0][cheat_start_loc.1];
                                let cheat_end_length =
                                    distance_ord[cheat_end_loc.0][cheat_end_loc.1];
                                let cheat_length = x_delta.abs() + y_delta.abs();
                                cheat_start_length.abs_diff(cheat_end_length) as i64
                                    >= 100 + cheat_length as i64
                            } else {
                                false
                            }
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u64
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
