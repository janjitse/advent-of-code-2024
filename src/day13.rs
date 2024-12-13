use regex::Regex;
use std::time::SystemTime;

#[derive(Debug)]
struct Entry {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse(input: &str) -> Vec<Entry> {
    let time_start = SystemTime::now();
    let lines = input.lines().collect::<Vec<_>>();
    let mut full_output = Vec::with_capacity(lines.len() / 4);
    let regex = Regex::new(r"X(?:=|\+)(?<x>\d+), Y(?:=|\+)(?<y>\d+)").unwrap();
    for l in lines.chunks(4) {
        let buttons_prize: Vec<_> = l
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| {
                let cap = regex.captures(x).unwrap();
                (
                    cap["x"].parse::<u64>().unwrap(),
                    cap["y"].parse::<u64>().unwrap(),
                )
            })
            .collect();
        let total = Entry {
            button_a: buttons_prize[0],
            button_b: buttons_prize[1],
            prize: buttons_prize[2],
        };
        full_output.push(total);
    }

    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    full_output
}

#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut total = 0;
    for entry in x {
        let det = (entry.button_a.0 * entry.button_b.1) as i64
            - (entry.button_a.1 * entry.button_b.0) as i64;
        assert_ne!(det, 0);
        let raw_a = entry.prize.0 as i64 * entry.button_b.1 as i64
            - entry.button_b.0 as i64 * entry.prize.1 as i64;
        let raw_b = entry.prize.1 as i64 * entry.button_a.0 as i64
            - entry.button_a.1 as i64 * entry.prize.0 as i64;
        if raw_a % det == 0
            && raw_b % det == 0
            && raw_a / det >= 0
            && raw_b / det >= 0
            && raw_a / det <= 100
            && raw_b / det <= 100
        {
            total += (3 * raw_a / det + raw_b / det) as u64;
        }
    }
    total
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut total = 0;
    for mut entry in x {
        entry.prize.0 += 10000000000000;
        entry.prize.1 += 10000000000000;
        let det = (entry.button_a.0 * entry.button_b.1) as i64
            - (entry.button_a.1 * entry.button_b.0) as i64;
        assert_ne!(det, 0);
        let raw_a = entry.prize.0 as i64 * entry.button_b.1 as i64
            - entry.button_b.0 as i64 * entry.prize.1 as i64;
        let raw_b = entry.prize.1 as i64 * entry.button_a.0 as i64
            - entry.button_a.1 as i64 * entry.prize.0 as i64;
        if raw_a % det == 0 && raw_b % det == 0 && raw_a / det >= 0 && raw_b / det >= 0 {
            total += (3 * raw_a / det + raw_b / det) as u64;
        }
    }
    total
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
        assert_eq!(part1(&contents), 480);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 875318608908);
    }
}
