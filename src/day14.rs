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

use crate::statistics::entropy;

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let mut x = parse(input);

    let width = 101;
    let height = 103;
    let mut t = 0;
    let mut min_entropy = f64::MAX;
    let mut min_t = 0;
    loop {
        t += 1;
        let mut new_positions = vec![];
        for (p, v) in x.into_iter() {
            let mut new_pos = ((p.0 + v.0) % width, (p.1 + v.1) % height);
            if new_pos.0 < 0 {
                new_pos = (new_pos.0 + width, new_pos.1);
            }
            if new_pos.1 < 0 {
                new_pos = (new_pos.0, new_pos.1 + height);
            }
            new_positions.push((new_pos, v));
        }
        x = new_positions;

        if t == (width * height) as u64 {
            break;
        }
        let mut flattened = vec![0u8; (width as usize * height as usize) / 6];
        for (p, _) in x.iter() {
            let loc_in_flattened = ((p.0 * width + p.1) / 6) as usize;
            let loc_in_u6 = (p.0 * width + p.1) % 6;
            flattened[loc_in_flattened] += 1 << loc_in_u6;
        }

        let entr = entropy(&flattened);
        if entr < min_entropy {
            min_entropy = entr;
            min_t = t;
            // println!("{:?},{:?}", t, entr);
        }
    }
    min_t
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
