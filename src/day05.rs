fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut output1: Vec<Vec<i32>> = vec![];
    let mut output2: Vec<Vec<i32>> = vec![];
    let input_split: Vec<&str> = input.split("\n\n").collect();

    for l in input_split[0].lines() {
        let mut temp: Vec<i32> = vec![];
        for t in l.split("|") {
            temp.push(t.parse().unwrap());
        }
        output1.push(temp);
    }
    for l in input_split[1].lines() {
        let temp = l
            .split(",")
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();
        output2.push(temp);
    }
    return (output1, output2);
}

use std::collections::{HashMap, HashSet};

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let (x1, x2) = parse(input);
    let mut ordering_before = HashMap::new();
    let mut ordering_after = HashMap::new();
    for x in x1.into_iter() {
        ordering_before
            .entry(x[0])
            .or_insert(HashSet::new())
            .insert(x[1]);
        ordering_after
            .entry(x[1])
            .or_insert(HashSet::new())
            .insert(x[0]);
    }
    let mut output = 0;
    for y in x2 {
        let mut is_possible = true;
        for (idx, val) in y.iter().enumerate() {
            match ordering_after.get(val) {
                None => {
                    continue;
                }
                Some(t) => {
                    for next_idx in idx..y.len() {
                        if t.contains(&y[next_idx]) {
                            println!("{:?} should come before {:?}", y[next_idx], *val);
                            is_possible = false;
                            break;
                        }
                    }
                }
            }
        }
        if is_possible {
            println!("{:?}", y);
            let mid_idx = y.len() / 2;
            output += y[mid_idx];
        }
    }
    return output;
}
use std::cmp::Ordering;

fn cmp_cust(&a: &i32, &b: &i32, ordering_after: &HashMap<i32, HashSet<i32>>) -> Ordering {
    match ordering_after.get(&a) {
        None => {}
        Some(t) => {
            if t.contains(&b) {
                return Ordering::Greater;
            }
        }
    }
    return Ordering::Less;
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let (x1, mut x2) = parse(input);
    let mut ordering_before = HashMap::new();
    let mut ordering_after = HashMap::new();
    for x in x1.into_iter() {
        ordering_before
            .entry(x[0])
            .or_insert(HashSet::new())
            .insert(x[1]);
        ordering_after
            .entry(x[1])
            .or_insert(HashSet::new())
            .insert(x[0]);
    }
    let mut output = 0;
    for y in x2.iter_mut() {
        let y_before = y.clone();
        println!("{:?}", y);
        y.sort_by(|a, b| cmp_cust(a, b, &ordering_after));
        println!("{:?}", y);
        let mid_point = y.len() / 2;
        if &y_before != y {
            output += y[mid_point];
        }
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
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 143);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 123);
    }
}
