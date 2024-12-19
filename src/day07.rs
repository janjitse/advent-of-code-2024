use std::time::SystemTime;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let l: Vec<&str> = line.split(':').collect();
            let left: u64 = l[0].parse().unwrap();
            let right: Vec<u64> = l[1]
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (left, right)
        })
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)] 
// #[aoc(day7, part1, slow)]
fn part1(input: &str) -> u64 {
    let s = parse(input);
    let mut output = 0;
    for (target, todo_vec) in s {
        let comb = 2u64.pow(todo_vec.len() as u32 - 1);
        for trial in 0..comb {
            let trial_digits: Vec<u64> =
                (0..todo_vec.len() - 1).map(|n| (trial >> n) & 1).collect();
            let mut outcome = todo_vec[0];
            for idx in 0..trial_digits.len() {
                match trial_digits[idx] {
                    0 => outcome += todo_vec[idx + 1],
                    1 => outcome *= todo_vec[idx + 1],
                    _ => {}
                }
                if outcome > target {
                    break;
                }
            }
            if outcome == target {
                output += target;
                break;
            }
        }
    }
    output
}

#[aoc(day7, part1, recurse)]
fn part1_rec(input: &str) -> u64 {
    parse(input)
        .into_iter()
        .filter(|(target, todo_vec)| recurse_part_a(*target, todo_vec))
        .map(|(target, _)| target)
        .sum()
}

fn recurse_part_a(target: u64, todo_vec: &[u64]) -> bool {
    if todo_vec.len() == 1 {
        return todo_vec[0] == target;
    }
    let (next_trial, todo_vec_new) = todo_vec.split_last().unwrap();
    if *next_trial > 0 {
        if target % next_trial == 0 {
            let multi_pos = recurse_part_a(target / next_trial, todo_vec_new);
            if multi_pos {
                return true;
            }
        }
    } else if target == 0 {
        return true;
    }
    if *next_trial <= target {
        return recurse_part_a(target - next_trial, todo_vec_new)
    }
    false    
}

#[aoc(day7, part2, recurse)]
fn part2_rec(input: &str) -> u64 {
    parse(input)
        .into_iter()
        .filter(|(target, todo_vec)| recurse_part_b(*target, todo_vec))
        .map(|(target, _)| target)
        .sum()
}

fn recurse_part_b(target: u64, todo_vec: &[u64]) -> bool {
    if todo_vec.len() == 1 {
        return todo_vec[0] == target;
    }
    let (next_trial, todo_vec_new) = todo_vec.split_last().unwrap();
    if *next_trial > 0 {
        if target % next_trial == 0 {
            let can_do = recurse_part_b(target / next_trial, todo_vec_new);
            if can_do {
                return true;
            }
        }
    } else if target == 0 {
        return true;
    }
    let trial_length = 10u64.pow(next_trial.checked_ilog10().unwrap_or(0) + 1);
    if target % trial_length == *next_trial {
        let can_do = recurse_part_b(target / trial_length, todo_vec_new);
        if can_do {
            return true;
        }
    }
    if *next_trial <= target {
        return recurse_part_b(target - next_trial, todo_vec_new)
    }
    false    
}

fn generate_ternary(mut nr: u64, length: usize) -> Vec<u64> {
    let mut output = vec![];
    for _ in 0..length {
        output.push(nr % 3);
        nr /= 3;
    }
    output
}

#[allow(dead_code)]
// #[aoc(day7, part2, slow)]
fn part2(input: &str) -> u64 {
    let s = parse(input);
    let mut output = 0;
    for (target, todo_vec) in s {
        let comb = 3u64.pow(todo_vec.len() as u32 - 1);
        for trial in 0..comb {
            let trial_digits: Vec<u64> = generate_ternary(trial, todo_vec.len() - 1);
            let mut outcome = todo_vec[0];
            for idx in 0..trial_digits.len() {
                match trial_digits[idx] {
                    0 => outcome += todo_vec[idx + 1],
                    1 => outcome *= todo_vec[idx + 1],
                    2 => {
                        outcome = outcome
                            * 10u64.pow(todo_vec[idx + 1].checked_ilog10().unwrap_or(0) + 1)
                            + todo_vec[idx + 1];
                    }
                    _ => {}
                }
                if outcome > target {
                    break;
                }
            }
            if outcome == target {
                output += target;
                break;
            }
        }
    }
    output
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
        assert_eq!(part1(&contents), 3749);
    }

    #[test]
    fn test_1_rec() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1_rec(&contents), 3749);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 11387);
    }

    #[test]
    fn test_2_rec() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2_rec(&contents), 11387);
    }

    #[test]
    fn test_2_rec_edge() {
        let contents = "31435833265: 7 691 3 754 14 56 8 59 81 950 923 1 530 3 35 233 514 1 92 44 0 168 31 4 268 47 23 6 4 3 318 9 41 0 6 2 28 1 98 1 8 443 8 54 41 4 25 729 7 82 341 2 5 825";
        assert_eq!(part1_rec(&contents), 31435833265)
    }
}
