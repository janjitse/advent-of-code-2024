use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    let time_start = SystemTime::now();
    let lines = input.lines().collect::<Vec<_>>();
    let mut output = vec![];
    for l in lines.chunks(8) {
        let output1 = l
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        output.push(output1);
    }
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output
}

#[aoc(day25, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut keys = vec![];
    let mut locks = vec![];
    for unk in x {
        if unk[0] == vec!['#', '#', '#', '#', '#'] {
            // lock
            let mut lock_depths = vec![];
            for col in 0..unk[0].len() {
                for (row, item) in unk.iter().enumerate() {
                    if item[col] == '.' {
                        lock_depths.push(row - 1);
                        break;
                    }
                }
            }
            locks.push(lock_depths);
        } else {
            let mut key_heights = vec![];
            for col in 0..unk[0].len() {
                for (row, item) in unk.iter().enumerate() {
                    if item[col] == '#' {
                        key_heights.push(6 - row);
                        break;
                    }
                }
            }
            keys.push(key_heights);
        }
    }
    let mut count = 0;
    for k in keys.iter() {
        for l in locks.iter() {
            let mut possible = true;
            for (k_v, l_v) in k.iter().zip(l.iter()) {
                if k_v + l_v >= 6 {
                    possible = false;
                    break;
                }
            }
            if possible {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
// #[aoc(day25, part2)]
fn part2(_input: &str) -> String {
    todo!()
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
        assert_eq!(part1(&contents), 3);
    }
}
