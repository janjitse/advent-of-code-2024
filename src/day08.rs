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

use rustc_hash::{FxHashMap, FxHashSet};

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let x = parse(input);
    let mut signals: FxHashMap<char, Vec<(usize, usize)>> = FxHashMap::default();
    for row_idx in 0..x.len() {
        for col_idx in 0..x[0].len() {
            if x[row_idx][col_idx] != '.' {
                let c = x[row_idx][col_idx];
                signals.entry(c).or_default().push((row_idx, col_idx));
            }
        }
    }
    let mut antipodes: FxHashSet<(usize, usize)> = FxHashSet::default();
    for locations in signals.into_values() {
        for first_loc in locations.iter() {
            for second_loc in locations.iter() {
                if first_loc != second_loc {
                    let antipode_pos_1 = (
                        (2 * second_loc.0).overflowing_sub(first_loc.0).0,
                        (2 * second_loc.1).overflowing_sub(first_loc.1).0,
                    );
                    if antipode_pos_1.0 < x.len() && antipode_pos_1.1 < x[0].len() {
                        antipodes.insert(antipode_pos_1);
                    }
                }
            }
        }
    }
    antipodes.len() as i32
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let x = parse(input);
    let mut signals: FxHashMap<char, Vec<(usize, usize)>> = FxHashMap::default();
    for row_idx in 0..x.len() {
        for col_idx in 0..x[0].len() {
            if x[row_idx][col_idx] != '.' {
                let c = x[row_idx][col_idx];
                signals.entry(c).or_default().push((row_idx, col_idx));
            }
        }
    }
    let mut antipodes: FxHashSet<(isize, isize)> = FxHashSet::default();
    for locations in signals.into_values() {
        for first_loc in locations.iter() {
            for second_loc in locations.iter() {
                if first_loc == second_loc {
                    continue;
                }
                let mut antipode_dir = (
                    second_loc.0 as isize - first_loc.0 as isize,
                    second_loc.1 as isize - first_loc.1 as isize,
                );

                let divisor = gcd(antipode_dir.0.abs(), antipode_dir.1.abs());
                if divisor > 1 {
                    println!("{:?} occurs", divisor);
                }
                antipode_dir = (antipode_dir.0 / divisor, antipode_dir.1 / divisor);
                let mut next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                while next_pos.0 < x.len() as isize
                    && 0 <= next_pos.0
                    && 0 <= next_pos.1
                    && next_pos.1 < x[0].len() as isize
                {
                    antipodes.insert(next_pos);
                    next_pos = (next_pos.0 + antipode_dir.0, next_pos.1 + antipode_dir.1);
                }
            }
        }
    }
    // println!("{:?}",antipodes);
    // for row_idx in 0..x.len() {
    //     for col_idx in 0..x[0].len() {
    //         if antipodes.contains(&(row_idx as isize, col_idx as isize)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }
    antipodes.len() as i32
}

fn gcd(mut n: isize, mut m: isize) -> isize {
    if m == 0 {
        return n;
    } else if n == 0 {
        return m;
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
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
        assert_eq!(part1(&contents), 14);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 34);
    }
}
