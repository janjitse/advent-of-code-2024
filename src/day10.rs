use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<u32>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use std::collections::{HashSet, VecDeque};

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let x = parse(input);
    let mut trailheads = HashSet::new();
    let mut trail_ends = HashSet::new();
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 0 {
                trailheads.insert((row_idx, col_idx));
            } else if col == 9 {
                trail_ends.insert((row_idx, col_idx));
            }
        }
    }
    let directions = [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)];
    let mut output = 0;
    for start in trailheads {
        let mut visited = HashSet::new();
        let mut todo = VecDeque::new();
        todo.push_back((start, 0));
        while let Some((next_node, height)) = todo.pop_front() {
            for dir in directions {
                if next_node.0.wrapping_add(dir.0) < x.len()
                    && next_node.1.wrapping_add(dir.1) < x[0].len()
                {
                    let cand = (
                        next_node.0.wrapping_add(dir.0),
                        next_node.1.wrapping_add(dir.1),
                    );
                    if height == 8 && x[cand.0][cand.1] == 9 {
                        visited.insert(cand);
                        continue;
                    }
                    if x[cand.0][cand.1] == height + 1 {
                        todo.push_back((cand, height + 1));
                    }
                }
            }
        }
        // println!("trailhead {:?}, score {:?}", start, visited.len() );
        output += visited.len() as i32;
    }
    return output;
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut trailheads = HashSet::new();
    let mut trail_ends = HashSet::new();
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == 0 {
                trailheads.insert((row_idx, col_idx));
            } else if col == 9 {
                trail_ends.insert((row_idx, col_idx));
            }
        }
    }
    let directions = [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)];
    let mut output = 0;
    for start in trailheads {
        let mut trails = HashSet::new();
        let mut todo = VecDeque::new();
        todo.push_back((start, 0, vec![start]));
        while let Some((next_node, height, trail_so_far)) = todo.pop_front() {
            for dir in directions {
                if next_node.0.wrapping_add(dir.0) < x.len()
                    && next_node.1.wrapping_add(dir.1) < x[0].len()
                {
                    let cand = (
                        next_node.0.wrapping_add(dir.0),
                        next_node.1.wrapping_add(dir.1),
                    );
                    if height == 8 && x[cand.0][cand.1] == 9 {
                        let mut new_trail = trail_so_far.clone();
                        new_trail.push(cand);
                        trails.insert(new_trail);
                        continue;
                    }
                    if x[cand.0][cand.1] == height + 1 {
                        let mut new_trail = trail_so_far.clone();
                        new_trail.push(cand);
                        todo.push_back((cand, height + 1, new_trail));
                    }
                }
            }
        }
        output += trails.len() as u64;
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
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 36);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 81);
    }
}
