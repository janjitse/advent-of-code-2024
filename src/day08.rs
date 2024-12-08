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
    return output1;
}

use std::collections::{HashMap, HashSet};

#[aoc(day8, part1)]
fn part1(input: &str) -> i32 {
    let x = parse(input);
    let mut signals: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row_idx in 0..x.len() {
        for col_idx in 0..x[0].len() {
            if x[row_idx][col_idx] != '.' {
                let c = x[row_idx][col_idx];
                signals.entry(c).or_insert(vec![]).push((row_idx, col_idx));
            }
        }
    }
    let mut antipodes: HashSet<(usize, usize)> = HashSet::new();
    for locations in signals.into_values() {
        for (first_idx, first_loc) in locations.iter().enumerate() {
            for (_, second_loc) in locations.iter().enumerate().skip(first_idx + 1) {
                let antipode_pos_1 = (
                    (2 * second_loc.0).overflowing_sub(first_loc.0).0,
                    (2 * second_loc.1).overflowing_sub(first_loc.1).0,
                );
                let antipode_pos_2 = (
                    (2 * first_loc.0).overflowing_sub(second_loc.0).0,
                    (2 * first_loc.1).overflowing_sub(second_loc.1).0,
                );
                if antipode_pos_1.0 < x.len() && antipode_pos_1.1 < x[0].len() {
                    antipodes.insert(antipode_pos_1);
                }
                if antipode_pos_2.0 < x.len() && antipode_pos_2.1 < x[0].len() {
                    antipodes.insert(antipode_pos_2);
                }
            }
        }
    }
    // println!("{:?}",antipodes);
    return antipodes.len() as i32;
}

#[aoc(day8, part2)]
fn part2(input: &str) -> i32 {
    let x = parse(input);
    let mut signals: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for row_idx in 0..x.len() {
        for col_idx in 0..x[0].len() {
            if x[row_idx][col_idx] != '.' {
                let c = x[row_idx][col_idx];
                signals.entry(c).or_insert(vec![]).push((row_idx, col_idx));
            }
        }
    }
    let mut antipodes: HashSet<(isize, isize)> = HashSet::new();
    for locations in signals.into_values() {
        for (first_idx, first_loc) in locations.iter().enumerate() {
            for (_, second_loc) in locations.iter().enumerate().skip(first_idx + 1) {
                let antipode_dir = (
                    second_loc.0 as isize - first_loc.0 as isize,
                    second_loc.1 as isize - first_loc.1 as isize,
                );
                match antipode_dir {
                    (0, 0) => {}
                    (0, t) => {
                        let mut next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0, next_pos.1 + 1);
                        }
                        next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0, next_pos.1 - 1);
                        }
                    }
                    (t, 0) => {
                        let mut next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0 + 1, next_pos.1);
                        }
                        next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0 - 1, next_pos.1);
                        }
                    }
                    (mut s, mut t) => {
                        let g = gcd(s.abs(), t.abs());
                        s /= g;
                        t /= g;
                        let mut next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0 + s, next_pos.1 + t);
                        }
                        next_pos = (first_loc.0 as isize, first_loc.1 as isize);
                        while next_pos.0 < x.len() as isize
                            && 0 <= next_pos.0
                            && 0 <= next_pos.1
                            && next_pos.1 < x[0].len() as isize
                        {
                            antipodes.insert(next_pos);
                            next_pos = (next_pos.0 - s, next_pos.1 - t);
                        }
                    }
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
    return antipodes.len() as i32;
}

fn gcd(mut n: isize, mut m: isize) -> isize {
    assert!(n != 0 && m != 0);
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
