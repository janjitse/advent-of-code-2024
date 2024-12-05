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
    let mut ordering_after = HashMap::new();
    for x in x1.into_iter() {
        ordering_after
            .entry(x[1])
            .or_insert(HashSet::new())
            .insert(x[0]);
    }
    let mut output = 0;
    for y in x2 {
        let mut is_possible = true;
        for left_right in y.windows(2) {
            if cmp_cust(&left_right[0], &left_right[1], &ordering_after) == Ordering::Greater {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            let mid_idx = y.len() / 2;
            output += y[mid_idx];
        }
    }
    return output;
}
use std::cmp::Ordering;

fn cmp_cust(a: &i32, b: &i32, ordering_after: &HashMap<i32, HashSet<i32>>) -> Ordering {
    match ordering_after.get(a) {
        None => {
            return Ordering::Less;
        }
        Some(t) => {
            if t.contains(b) {
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
    }
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let (x1, mut x2) = parse(input);
    let mut ordering_after = HashMap::new();
    for x in x1.into_iter() {
        ordering_after
            .entry(x[1])
            .or_insert(HashSet::new())
            .insert(x[0]);
    }
    let mut output = 0;
    for y in x2.iter_mut() {
        let mut is_possible = true;
        for left_right in y.windows(2) {
            if cmp_cust(&left_right[0], &left_right[1], &ordering_after) == Ordering::Greater {
                is_possible = false;
                break;
            }
        }
        if !is_possible {
            let mid_idx = y.len() / 2;
            output += *y
                .select_nth_unstable_by(mid_idx, |a, b| cmp_cust(a, b, &ordering_after))
                .1;
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
