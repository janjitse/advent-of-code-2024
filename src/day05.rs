fn parse(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split('|')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    (output1, output2)
}

use rustc_hash::{FxHashMap, FxHashSet};

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let (x1, x2) = parse(input);
    let mut ordering_after: FxHashMap<i32, FxHashSet<i32>> = FxHashMap::default();
    let mut ordering_before: FxHashMap<i32, FxHashSet<i32>> = FxHashMap::default();
    for x in x1.into_iter() {
        ordering_after.entry(x[1]).or_default().insert(x[0]);
        ordering_before.entry(x[0]).or_default().insert(x[1]);
    }
    let mut output = 0;
    for y in x2 {
        let mut is_possible = true;
        for left_right in y.windows(2) {
            if cmp_cust(
                &left_right[0],
                &left_right[1],
                &ordering_after,
                &ordering_before,
            ) == Ordering::Greater
            {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            let mid_idx = y.len() / 2;
            output += y[mid_idx];
        }
    }
    output
}

use std::cmp::Ordering;

fn cmp_cust(
    a: &i32,
    b: &i32,
    ordering_after: &FxHashMap<i32, FxHashSet<i32>>,
    ordering_before: &FxHashMap<i32, FxHashSet<i32>>,
) -> Ordering {
    match ordering_after.get(a) {
        None => {}
        Some(t) => {
            if t.contains(b) {
                return Ordering::Greater;
            }
        }
    }
    match ordering_before.get(a) {
        None => {}
        Some(t) => {
            if t.contains(b) {
                return Ordering::Less;
            }
        }
    }
    println!("{:?},{:?}", *a, *b);
    unreachable!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let (x1, mut x2) = parse(input);
    let mut ordering_after: FxHashMap<i32, FxHashSet<i32>> = FxHashMap::default();
    let mut ordering_before: FxHashMap<i32, FxHashSet<i32>> = FxHashMap::default();
    for x in x1.into_iter() {
        ordering_after.entry(x[1]).or_default().insert(x[0]);
        ordering_before.entry(x[0]).or_default().insert(x[1]);
    }
    let mut output = 0;
    for y in x2.iter_mut() {
        let mut is_possible = true;
        for left_right in y.windows(2) {
            if cmp_cust(
                &left_right[0],
                &left_right[1],
                &ordering_after,
                &ordering_before,
            ) == Ordering::Greater
            {
                is_possible = false;
                break;
            }
        }
        if !is_possible {
            let mid_idx = y.len() / 2;
            output += *y
                .select_nth_unstable_by(mid_idx, |a, b| {
                    cmp_cust(a, b, &ordering_after, &ordering_before)
                })
                .1;
        }
    }
    output
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
