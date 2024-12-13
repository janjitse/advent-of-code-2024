use std::time::SystemTime;
use regex::Regex;

#[derive(Debug)]
struct Entry {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse(input: &str) -> Vec<Entry> {
    let time_start = SystemTime::now();
    let lines = input.split("\r\n\r\n");
    let mut full_output = vec![];
    let regex = Regex::new(r"X(?:=|\+)(\d+), Y(?:=|\+)(\d+)").unwrap();
    for l in lines {
        let mut button_a:(u64, u64) = (0,0);
        let mut button_b:(u64, u64) = (0,0);
        let mut prize:(u64, u64) = (0,0);
        for (idx, p) in l.split("\r\n").enumerate() {
            if idx == 0 {
                let captures = regex.captures(p).unwrap();
                button_a = (captures[1].parse::<u64>().unwrap(), captures[2].parse::<u64>().unwrap());
            }
            if idx == 1 {
                let captures = regex.captures(p).unwrap();
                button_b = (captures[1].parse::<u64>().unwrap(), captures[2].parse::<u64>().unwrap());

            }
            if idx == 2 {
                let captures = regex.captures(p).unwrap();
                prize = (captures[1].parse::<u64>().unwrap(), captures[2].parse::<u64>().unwrap());
            }


        }
        let total = Entry {
            button_a, button_b, prize
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
        let det = (entry.button_a.0 * entry.button_b.1) as i64 - (entry.button_a.1 * entry.button_b.0) as i64;
        assert_ne!(det, 0);
        let raw_a = entry.prize.0 as i64 *  entry.button_b.1 as i64 - entry.button_b.0 as i64 * entry.prize.1 as i64;
        let raw_b = entry.prize.1 as i64 *  entry.button_a.0 as i64 - entry.button_a.1 as i64 * entry.prize.0 as i64;
        if raw_a % det == 0 && raw_b % det == 0 {
            if raw_a / det >= 0 && raw_b / det >= 0  && raw_a / det <= 100 && raw_b / det <= 100 {
                total += (3 * raw_a / det + raw_b / det) as u64; 
            }
            
        }
    }
    return total
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut total = 0;
    for mut entry in x {
        entry.prize.0 += 10000000000000;
        entry.prize.1 += 10000000000000;
        let det = (entry.button_a.0 * entry.button_b.1) as i64 - (entry.button_a.1 * entry.button_b.0) as i64;
        assert_ne!(det, 0);
        let raw_a = entry.prize.0 as i64 *  entry.button_b.1 as i64 - entry.button_b.0 as i64 * entry.prize.1 as i64;
        let raw_b = entry.prize.1 as i64 *  entry.button_a.0 as i64 - entry.button_a.1 as i64 * entry.prize.0 as i64;
        if raw_a % det == 0 && raw_b % det == 0 {
            if raw_a / det >= 0 && raw_b / det >= 0 { // && raw_a / det <= 100 && raw_b / det <= 100 {
                total += (3 * raw_a / det + raw_b / det) as u64; 
            }
            
        }
    }
    return total
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