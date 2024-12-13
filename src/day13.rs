use std::time::SystemTime;
use regex::Regex;

#[derive(Debug,PartialEq)]
struct Entry {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

fn parse(input: &str) -> Vec<Entry> {
    let time_start = SystemTime::now();
    let lines = input.split("\r\n\r\n");
    let mut full_output = vec![];
    let regex_a = Regex::new(r"Button A: X\+(?<first>\d+), Y\+(?<last>\d+)").unwrap();
    let regex_b = Regex::new(r"Button B: X\+(?<first>\d+), Y\+(?<last>\d+)").unwrap();
    let regex_prize = Regex::new(r"Prize: X=(?<first>\d+), Y=(?<last>\d+)").unwrap();
    for l in lines {
        let mut button_a:(u64, u64) = (0,0);
        let mut button_b:(u64, u64) = (0,0);
        let mut prize:(u64, u64) = (0,0);
        for (idx, p) in l.split("\r\n").enumerate() {
            // println!("{:?}, {:?}", idx, p);
            
            if idx == 0 {
                // button_a = regex_a.find_iter(p).next().unwrap().
                // println!("{:?}", regex_a.captures(p).map(|x| );
                let captures = regex_a.captures(p).unwrap();
                button_a = (captures["first"].parse::<u64>().unwrap(), captures["last"].parse::<u64>().unwrap());

                // button_a = regex_a.captures(p).map(|x| x.extract().1.map(|y| y.parse().unwrap())).unwrap();
            }
            if idx == 1 {
                let captures = regex_b.captures(p).unwrap();
                button_b = (captures["first"].parse::<u64>().unwrap(), captures["last"].parse::<u64>().unwrap());

            }
            if idx == 2 {
                let captures = regex_prize.captures(p).unwrap();

                prize = (captures["first"].parse::<u64>().unwrap(), captures["last"].parse::<u64>().unwrap());
            }


        }
        // println!("{:?}, {:?}, {:?}", button_a, button_b, prize);
        let total = Entry {
            button_a, button_b, prize
        };
        // println!("{:?}", total);
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
        let mut min_cost = u64::MAX;
        for times_a in 0..=100 {
            for times_b in 0..=100 {
                if times_a * entry.button_a.0  + times_b * entry.button_b.0 == entry.prize.0 && 
                    times_a * entry.button_a.1  + times_b * entry.button_b.1 == entry.prize.1 {
                    let cost = 3 * times_a + times_b;
                    if (cost as u64) < min_cost {
                        min_cost = cost as u64;
                    }
                }
            }
        }
        if min_cost < u64::MAX {
            total += min_cost;
        }
    } 
    return total
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u128 {
    let x = parse(input);
    let mut total = 0;
    for mut entry in x {
        entry.prize.0 += 10000000000000;
        entry.prize.1 += 10000000000000;
        let det = (entry.button_a.0 * entry.button_b.1) as i128 - (entry.button_a.1 * entry.button_b.0) as i128;
        assert_ne!(det, 0);
        let raw_a = entry.prize.0 as i128 *  entry.button_b.1 as i128 - entry.button_b.0 as i128 * entry.prize.1 as i128;
        let raw_b = entry.prize.1 as i128 *  entry.button_a.0 as i128 - entry.button_a.1 as i128 * entry.prize.0 as i128;
        if raw_a % det == 0 && raw_b % det == 0 {
            if raw_a / det >= 0 && raw_b / det >= 0 { // && raw_a / det <= 100 && raw_b / det <= 100 {
                total += (3 * raw_a / det + raw_b / det) as u128; 
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
        assert_eq!(part2(&contents), -1);
    }

}