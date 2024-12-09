use std::time::SystemTime;

fn parse(input: &str) -> Vec<u32> {
    let time_start = SystemTime::now();
    let output1 = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[aoc(day9, part1)]
fn part1(input: &str) -> u128 {
    let x = parse(input);
    let mut checksum_vec = vec![];
    for (idx, file_len) in x.iter().enumerate() {
        if idx % 2 == 0 {
            for _ in 0..*file_len {
                checksum_vec.push((idx / 2) as isize);
            }
        } else {
            for _ in 0..*file_len {
                checksum_vec.push(-1);
            }
        }
    }
    let mut compacted_vec = vec![];
    let mut backpointer = checksum_vec.len() - 1;
    for idx in 0..checksum_vec.len() {
        if checksum_vec[idx] >= 0 {
            compacted_vec.push(checksum_vec[idx]);
        } else {
            while checksum_vec[backpointer] < 0 {
                backpointer -= 1;
            }
            if backpointer <= idx {
                break;
            }
            compacted_vec.push(checksum_vec[backpointer]);
            backpointer -= 1;
        }
        if backpointer <= idx {
            break;
        }
    }
    let mut output = 0;
    for (idx, val) in compacted_vec.into_iter().enumerate() {
        output += val as u128 * idx as u128
    }
    output
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct File {
    start: Reverse<usize>,
    end: usize,
    id: isize,
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u128 {
    let x = parse(input);
    let mut files: Vec<File> = vec![];
    let mut current_end = 0;
    let mut freespaces: [BinaryHeap<File>; 9] = [
        BinaryHeap::with_capacity(x.len() / 18), // 1
        BinaryHeap::with_capacity(x.len() / 18), // 2
        BinaryHeap::with_capacity(x.len() / 18), // 3
        BinaryHeap::with_capacity(x.len() / 18), // 4
        BinaryHeap::with_capacity(x.len() / 18), // 5
        BinaryHeap::with_capacity(x.len() / 18), // 6
        BinaryHeap::with_capacity(x.len() / 18), // 7
        BinaryHeap::with_capacity(x.len() / 18), // 8
        BinaryHeap::with_capacity(x.len() / 18), // 9
    ];

    for (idx, file_len) in x.iter().enumerate() {
        if idx % 2 == 0 {
            let start = current_end;
            let end = start + *file_len as usize;
            files.push(File {
                start: Reverse(start),
                end,
                id: (idx / 2) as isize,
            });
            current_end = end;
        } else {
            if *file_len == 0 {
                continue;
            }
            let start = current_end;
            let end = start + *file_len as usize;
            freespaces[*file_len as usize - 1].push(File {
                start: Reverse(start),
                end,
                id: -1,
            });
            current_end = end;
        }
    }

    let mut compacted_files: Vec<File> = Vec::with_capacity(files.len());
    while let Some(next_file) = files.pop() {
        match find_next_free(
            next_file.end - next_file.start.0,
            &mut freespaces,
            next_file.start.0,
        ) {
            None => {
                compacted_files.push(next_file);
            }
            Some(free_file) => {
                let new_file = File {
                    start: free_file.start,
                    end: free_file.start.0 + next_file.end - next_file.start.0,
                    id: next_file.id,
                };
                if new_file.end < free_file.end {
                    let len = free_file.end - new_file.end;
                    let new_free = File {
                        start: Reverse(new_file.end),
                        end: free_file.end,
                        id: -1,
                    };
                    freespaces[len - 1].push(new_free);
                }
                compacted_files.push(new_file);
            }
        }
    }
    let mut checksum = 0;
    for c in compacted_files {
        for idx in c.start.0..c.end {
            checksum += idx as u128 * c.id as u128;
        }
    }
    checksum
}

fn find_next_free(
    file_len: usize,
    freespaces: &mut [BinaryHeap<File>],
    max_start: usize,
) -> Option<File> {
    let mut min_start = usize::MAX;
    let mut min_start_len = 0;
    for size in file_len..=freespaces.len() {
        match freespaces[size - 1].peek() {
            None => {}
            Some(f) => {
                if f.start.0 < max_start && f.start.0 < min_start {
                    min_start = f.start.0;
                    min_start_len = size;
                }
            }
        }
    }
    if min_start < max_start {
        return freespaces[min_start_len - 1].pop();
    }
    None
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
        assert_eq!(part1(&contents), 1928);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 2858);
    }
}
