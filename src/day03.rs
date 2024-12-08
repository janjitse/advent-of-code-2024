use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    parse(input).iter().map(|x| x[0] * x[1]).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let (muls, do_pos, dont_pos, mul_pos) = parse_b(input);
    let mut outcome = 0;
    for (mul, pos) in muls.into_iter().zip(mul_pos) {
        let last_dos = *do_pos.iter().filter(|&&x| x < pos).last().unwrap_or(&0);
        let last_donts = *dont_pos.iter().filter(|&&x| x < pos).last().unwrap_or(&0);
        if last_dos >= last_donts {
            outcome += mul[0] * mul[1];
        }
    }
    outcome
}

#[aoc(day3, part2, cleaner)]
pub fn part2_cleaner(input: &str) -> i64 {
    let re = Regex::new(r"(mul|do|don't)\((([0-9]{1,3}),([0-9]{1,3}))?\)").unwrap();
    let mut on = true;
    let mut output = 0;
    for x in re.captures_iter(input) {
        match &x[1] {
            "do" => {
                on = true;
            }
            "don't" => {
                on = false;
            }
            "mul" => {
                if on {
                    let left: i32 = x[3].parse().unwrap();
                    let right: i32 = x[4].parse().unwrap();
                    output += (left * right) as i64;
                }
            }
            _ => {}
        }
    }
    output
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for line in input.lines() {
        for (_, [y, z]) in re.captures_iter(line).map(|x| x.extract()) {
            // println!("{:?}", x);
            let left: i32 = y.parse().unwrap();
            let right: i32 = z.parse().unwrap();
            output.push(vec![left, right]);
        }
    }
    output
}

fn parse_b(input: &str) -> (Vec<Vec<i32>>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut output = vec![];
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut do_pos = vec![];
    let mut dont_pos = vec![];
    let mut mul_pos = vec![];
    let line = input;
    for d in re_do.captures_iter(line).map(|x| x.get(0).unwrap().start()) {
        do_pos.push(d);
    }
    for d in re_dont
        .captures_iter(line)
        .map(|x| x.get(0).unwrap().start())
    {
        dont_pos.push(d);
    }
    for d in re.captures_iter(line).map(|x| x.get(0).unwrap().start()) {
        mul_pos.push(d);
    }
    for (_, [y, z]) in re.captures_iter(line).map(|x| x.extract()) {
        // println!("{:?}", x);
        let left: i32 = y.parse().unwrap();
        let right: i32 = z.parse().unwrap();
        output.push(vec![left, right]);
    }
    (output, do_pos, dont_pos, mul_pos)
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
        assert_eq!(part1(&contents), 161);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small_b.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 48);
    }
}
