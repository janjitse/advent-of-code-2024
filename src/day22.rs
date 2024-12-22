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

fn mix(result: u64, x: u64) -> u64 {
    result ^ x
}

fn prune(result: u64) -> u64 {
    result % 16777216
}

fn next_secret(mut secret: u64) -> u64 {
    secret = mix(secret << 6, secret);
    secret = prune(secret);

    secret = mix(secret >> 5, secret);
    secret = prune(secret);
    secret = mix(secret << 11, secret);
    prune(secret)
}
#[aoc(day22, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    println!("{:?}", x);
    let mut output = 0;
    for secret in x {
        let mut real_secret = secret;
        for _ in 0..2000 {
            real_secret = next_secret(real_secret);
        }
        println!("{:?}", real_secret);
        output += real_secret;
    }

    output
}

use rustc_hash::FxHashMap;

fn first_occurences(vector: Vec<i32>) -> FxHashMap<(i32, i32, i32, i32), i32> {
    let mut output = FxHashMap::default();
    for w in vector.windows(5) {
        let differences = (w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]);
        if !output.contains_key(&differences) {
            output.insert(differences, w[4]);
        }
    }
    output
}

#[aoc(day22, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    println!("{:?}", x);
    let mut all_first_differences = FxHashMap::default();
    for secret in x {
        let mut vec_digits = Vec::with_capacity(2001);
        vec_digits.push((secret % 10) as i32);
        let mut real_secret = secret;
        for _ in 0..2000 {
            real_secret = next_secret(real_secret);
            vec_digits.push((real_secret % 10) as i32);
        }
        let secret_differences = first_occurences(vec_digits);
        // println!("{:?}", secret_differences.get(&(-2,1,-1,3)));
        for (k, v) in secret_differences {
            all_first_differences.entry(k).or_insert(vec![]).push(v);
        }
    }
    let mut max_output = 0;
    // println!("{:?}", all_first_differences.get(&(-2,1,-1,3)));
    for (k, v) in all_first_differences {
        let output_sum: i32 = v.iter().sum();
        if output_sum as u64 > max_output {
            // println!("{:?}", k);
            max_output = output_sum as u64;
        }
    }
    return max_output;
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
