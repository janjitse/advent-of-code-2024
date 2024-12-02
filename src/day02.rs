#[aoc(day2, part1)]
pub fn part_a(contents: &str) -> i32 {
    let todo = parse(contents);
    let mut total_safe = 0;
    for t in todo {
        let safe: bool;
        if t[0] > *t.last().unwrap() {
            safe = check_all_descending(t);
        } else {
            safe = check_all_ascending(t);
        }
        if safe {
            total_safe += 1;
        }
    }

    return total_safe;
}

#[aoc(day2, part2)]
pub fn part_b(contents: &str) -> i32 {
    let mut todo = parse(contents);
    let mut total_safe = 0;

    for t in todo.iter_mut() {
        let mut safe = false;        
        if check_all_ascending(t.to_vec()) || check_all_descending(t.to_vec()) {
            safe = true;
        } else {
            for mask in 0..t.len() {
                let removed = t.remove(mask);
                if check_all_ascending(t.to_vec()) || check_all_descending(t.to_vec()) {
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
    return total_safe;
}

fn check_all_descending(t: Vec<i32>) -> bool {
    let mut safe = true;
    for w in t.windows(2) {
        if w[0] <= w[1] || w[0] - 4 >= w[1] {
            safe = false;
            break;
        }
    }
    return safe;
}

fn check_all_ascending(t: Vec<i32>) -> bool {
    let mut safe = true;
    for w in t.windows(2) {
        if w[0] >= w[1] || w[1] - 4 >= w[0] {
            safe = false;
            break;
        }
    }
    return safe;
}

pub fn parse(contents: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    for line in contents.lines() {
        let vec: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        output.push(vec);
    }
    return output;
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
