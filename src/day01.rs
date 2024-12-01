use std::collections::HashMap;


#[aoc(day1, part1)]
pub fn part_a(contents: &str) -> i32 {
    let (mut vec1, mut vec2) = parse(contents);
    vec1.sort_unstable();
    vec2.sort_unstable();
    let total = vec1
        .iter()
        .zip(vec2.iter())
        .fold(0, |acc, (&x1, &x2)| acc + (x1 - x2).abs());
    return total;
}

#[aoc(day1, part2)]
pub fn part_b(contents: &str) -> i32 {
    let (vec1, vec2) = parse(contents);
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for val2 in vec2.into_iter() {
        *counter.entry(val2).or_insert(0) += 1;
    }
    let total2 = vec1
        .into_iter()
        .fold(0, |acc, x| acc + *counter.get(&x).unwrap_or(&0) * x);
    return total2;
}

pub fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let vec: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let vec1 = vec.iter().step_by(2).map(|x| *x).collect::<Vec<i32>>();
    let vec2 = vec
        .iter()
        .skip(1)
        .step_by(2)
        .map(|x| *x)
        .collect::<Vec<i32>>();
    return (vec1, vec2);
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
        assert_eq!(part_a(&contents), 11);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 31);
    }
}
