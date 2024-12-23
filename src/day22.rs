use std::time::SystemTime;

fn parse(input: &str) -> Vec<u64> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret << 6;
    secret %= 16777216;
    secret ^= secret >> 5;
    secret %= 16777216;
    secret ^= secret << 11;
    secret % 16777216
}

#[aoc(day22, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut output = 0;
    for secret in x {
        let mut real_secret = secret as u32;
        for _ in 0..2000 {
            real_secret = next_secret(real_secret);
        }
        output += real_secret as u64;
    }

    output
}

#[aoc(day22, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let size = 19 * 19 * 19 * 19;
    let modulus = 19 * 19 * 19;
    let mut all_first_differences = vec![0; size];
    let mut last_seen = vec![-1; size];

    for (sec_idx, secret) in x.into_iter().enumerate() {
        let mut last_secret = (secret % 10) as usize;
        let mut real_secret = secret as u32;
        let mut key = 0;
        for idx in 0..2000 {
            real_secret = next_secret(real_secret);
            let difference = (real_secret % 10) as usize + 9 - last_secret;
            key = (19 * (key % (modulus))) + difference;
            if idx > 3 && last_seen[key] < sec_idx as i32 {
                last_seen[key] = sec_idx as i32;
                all_first_differences[key] += (real_secret % 10) as u64;
            }
            last_secret = (real_secret % 10) as usize;
        }
    }
    all_first_differences.into_iter().max().unwrap()
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
        assert_eq!(part1(&contents), 37327623);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 23);
    }
}
