use regex::Regex;
use std::time::SystemTime;

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let time_start = SystemTime::now();
    let lines = input.lines();
    let mut total_output = vec![];
    let regex = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    for l in lines {
        let cap = regex.captures(l).unwrap();
        let p = (
            cap["px"].parse::<i32>().unwrap(),
            cap["py"].parse::<i32>().unwrap(),
        );
        let v = (
            cap["vx"].parse::<i32>().unwrap(),
            cap["vy"].parse::<i32>().unwrap(),
        );
        total_output.push((p, v));
    }
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    total_output
}

use std::collections::HashMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let mut x = parse(input);
    let width = 101;
    let height = 103;
    for _ in 0..100 {
        let mut new_positions = vec![];
        for (p, v) in x.iter() {
            let mut new_pos = ((p.0 + v.0) % width, (p.1 + v.1) % height);
            if new_pos.0 < 0 {
                new_pos = (new_pos.0 + width, new_pos.1);
            }
            if new_pos.1 < 0 {
                new_pos = (new_pos.0, new_pos.1 + height);
            }
            new_positions.push((new_pos, *v));
        }
        x = new_positions;
    }
    let mut quadrant_counts = (0, 0, 0, 0);

    for (p, _) in x {
        if p.0 < width / 2 && p.1 < height / 2 {
            quadrant_counts.0 += 1;
        }
        if p.0 > width / 2 && p.1 < height / 2 {
            quadrant_counts.1 += 1;
        }
        if p.0 < width / 2 && p.1 > height / 2 {
            quadrant_counts.2 += 1;
        }
        if p.0 > width / 2 && p.1 > height / 2 {
            quadrant_counts.3 += 1;
        }
    }

    println!("{:?}", quadrant_counts);
    quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut x = parse(input);

    let width = 101;
    let height = 103;
    let mut periods_hashmap = HashMap::new();
    let mut init_hashmap = HashMap::new();
    for (idx, (p, _)) in x.iter().enumerate() {
        init_hashmap.insert(idx, *p);
    }
    let mut t = 0;
    loop {
        t += 1;
        let mut new_positions = vec![];
        for (idx, (p, v)) in x.into_iter().enumerate() {
            let mut new_pos = ((p.0 + v.0) % width, (p.1 + v.1) % height);
            if new_pos.0 < 0 {
                new_pos = (new_pos.0 + width, new_pos.1);
            }
            if new_pos.1 < 0 {
                new_pos = (new_pos.0, new_pos.1 + height);
            }
            new_positions.push((new_pos, v));
            if *init_hashmap.get(&idx).unwrap() == new_pos {
                periods_hashmap.entry(idx).or_insert(t);
            }
        }
        x = new_positions;

        if x.len() == periods_hashmap.len() {
            break;
        }
        let mut vis: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
        for (p, _) in x.iter() {
            vis[p.1 as usize][p.0 as usize] = '#';
        }
        let mut found = false;
        for line in vis {
            if line
                .iter()
                .collect::<String>()
                .contains("#############################")
            {
                found = true;
            }
        }
        if found {
            break;
        }
    }
    t
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
        assert_eq!(part1(&contents), 12);
    }

    #[test]
    fn test_1_tiny() {
        // let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        // let file_path = format!("input/2024/{}_small.txt", s);
        // let contents = fs::read_to_string(file_path).expect("file not found");
        let contents = "p=2,4 v=2,-3".to_string();
        assert_eq!(part1(&contents), 12);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 0);
    }
}
