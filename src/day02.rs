#[aoc(day2, part1)]
pub fn part_a(contents: &str) -> i32 {
    parse(contents)
        .iter()
        .filter(|&t| check_all_descending(t) || check_all_ascending(t))
        .count() as i32
}

#[aoc(day2, part2, brute_force)]
pub fn part_b(contents: &str) -> i32 {
    let mut todo = parse(contents);
    let mut total_safe = 0;

    for t in todo.iter_mut() {
        let mut safe = false;
        if check_all_ascending(t) || check_all_descending(t) {
            safe = true;
        } else {
            for mask in 0..t.len() {
                let removed = t.remove(mask);
                if check_all_ascending(t) || check_all_descending(t) {
                    safe = true;
                    break;
                }
                t.insert(mask, removed);
            }
        }

        if safe {
            total_safe += 1;
        }
    }
    total_safe
}

#[aoc(day2, part2, recursing)]
pub fn part_b_recur(contents: &str) -> i32 {
    parse(contents)
        .iter()
        .filter(|&t| {
            check_all_descending_rec(t, 1)
                || check_all_descending_rec(&t.iter().copied().rev().collect::<Vec<i32>>(), 1)
        })
        .count() as i32
}

fn check_all_descending_rec(t: &[i32], mistakes_allowed: i32) -> bool {
    for (idx, w) in t.windows(2).enumerate() {
        if !((1 <= w[1] - w[0]) && (w[1] - w[0] <= 3)) {
            if mistakes_allowed > 0 {
                let mut orig_t = (*t).to_owned();
                let mut safe = false;
                for removal in [0, 1] {
                    let removed = orig_t.remove(idx + removal);
                    safe |= check_all_descending_rec(&orig_t, mistakes_allowed - 1);
                    orig_t.insert(idx + removal, removed)
                }
                return safe;
            } else {
                return false;
            }
        }
    }
    true
}

fn check_all_descending(t: &[i32]) -> bool {
    for w in t.windows(2) {
        if w[0] <= w[1] || w[0] - 4 >= w[1] {
            return false;
        }
    }
    true
}

fn check_all_ascending(t: &[i32]) -> bool {
    for w in t.windows(2) {
        if w[0] >= w[1] || w[1] - 4 >= w[0] {
            return false;
        }
    }
    true
}

pub fn parse(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
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
        assert_eq!(part_a(&contents), 2);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 4);
    }

    #[test]
    fn test_3() {
        let contents = "16 17 18 21 23 24 27 24".to_string();
        assert_eq!(part_b(&contents), 1);
    }
}
