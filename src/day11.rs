use std::time::SystemTime;

fn parse(input: &str) -> Vec<u128> {
    let time_start = SystemTime::now();
    let output1 = input
        .split_ascii_whitespace().map(|x| x.parse().unwrap())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut output = parse(input);
    for idx in 0..25 {
        let mut next_elem = vec![];
        let prev_len = output.len();
        for element in output {
            if element == 0 {
                next_elem.push(1)
            } else if (element.checked_ilog10().unwrap_or(0) + 1) %2 == 0 {
                let length = element.checked_ilog10().unwrap_or(0) + 1;
                next_elem.push(element / 10u128.pow(length/2) as u128);
                next_elem.push(element % 10u128.pow(length/2) as u128);
            } else {
                next_elem.push(element * 2024);
            }
        }
        println!("Step {:?}, {:?}, {:?}, {:?}", idx, next_elem.len(), next_elem.len() / prev_len, next_elem.len() % prev_len );  
        // println!("Step {:?}, {:?}", idx, next_elem);
        output = next_elem;
    }
    
    return output.len();
}

use fxhash::FxHashMap;

#[aoc(day11, part2)]
fn part2(input: &str) -> u128 {
    let mut output = parse(input);
    let mut hash = FxHashMap::default();
    for d in output {
        *hash.entry(d).or_insert(0 as u128) += 1;
    }
    for idx in 0..75 {
        let mut next_hash = FxHashMap::default();
        for (idx, amount) in hash {
            if idx == 0 {
                *next_hash.entry(1).or_insert(0) += amount;
            } else if (idx.checked_ilog10().unwrap_or(0) + 1) %2 == 0 {
                let length = idx.checked_ilog10().unwrap_or(0) + 1;
                *next_hash.entry(idx / 10u128.pow(length/2)).or_insert(0) += amount;
                *next_hash.entry(idx % 10u128.pow(length/2)).or_insert(0) += amount;
            } else {
                *next_hash.entry(idx * 2024).or_insert(0) += amount;
            }

        }
        // println!("Step {:?}, {:?}, {:?}, {:?}", idx, next_elem.len(), next_elem.len() / prev_len, next_elem.len() % prev_len );  
        // println!("Step {:?}, {:?}", idx, next_elem);
        // println!("Step {:?}, {:?}", idx, next_hash);
        hash = next_hash;
    }
    let mut total_length = 0;
    for d in hash.values() {
        total_length += d;
    }
    
    return total_length;
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
        assert_eq!(part1(&contents), 55312);
    }


    // #[test]
    // fn test_cycle() {
    //     let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    //     // let file_path = format!("input/2024/{}_small.txt", s);
    //     // let contents = fs::read_to_string(file_path).expect("file not found");
    //     let contents = "1".to_string();
    //     assert_eq!(part1(&contents), 1);
    // }
    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 55312);
    }
}