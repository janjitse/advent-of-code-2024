use std::iter;
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
fn part1(input: &str) -> i128 {
    let x = parse(input);
    let mut checksum_vec = Vec::with_capacity(6 * x.len());
    for (idx, file_len) in x.iter().enumerate() {
        let to_push = match idx % 2 {
            0 => (idx / 2) as isize,
            _ => -1,
        };
        checksum_vec.extend(iter::repeat(to_push).take(*file_len as usize));
    }
    let mut output = 0;
    let mut backpointer = checksum_vec.len() - 1;
    for idx in 0..checksum_vec.len() {
        if checksum_vec[idx] >= 0 {
            output += checksum_vec[idx] as i128 * idx as i128;
        } else {
            while backpointer > idx && checksum_vec[backpointer] < 0 {
                backpointer -= 1;
            }
            if checksum_vec[backpointer] >= 0 {
                output += idx as i128 * checksum_vec[backpointer] as i128;
            }
            backpointer -= 1;
        }
        if backpointer <= idx {
            break;
        }
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

#[allow(dead_code)]
// #[aoc(day9, part2)]
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

#[derive(Default, Debug)]
struct File2 {
    start: usize,
    len: usize,
    id: isize,
}

#[aoc(day9, part2, faster)]
fn part2_faster(input: &str) -> u128 {
    let x = parse(input);
    let mut files = [
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
        Vec::with_capacity(x.len() / 16),
    ];
    let mut current_end = 0;
    let mut empty_spaces: Vec<File2> = Vec::with_capacity(x.len() / 2);
    for (idx, &len) in x.iter().enumerate() {
        let start = current_end;
        if idx % 2 == 0 {
            files[len as usize - 1].push(File2 {
                start,
                len: len as usize,
                id: (idx / 2) as isize,
            });
        } else {
            if len == 0 {
                continue;
            }
            empty_spaces.push(File2 {
                start,
                len: len as usize,
                id: -1,
            });
        }
        current_end = start + len as usize;
    }
    let mut output = 0;
    for mut empty in empty_spaces {
        while empty.len > 0 {
            let mut right_most_idx = empty.start;
            let mut right_most_len = 0;
            for file_len in 1..=empty.len {
                if files[file_len - 1]
                    .last()
                    .unwrap_or(&File2::default())
                    .start
                    > right_most_idx
                {
                    right_most_idx = files[file_len - 1].last().unwrap().start;
                    right_most_len = file_len;
                }
            }
            if right_most_idx == empty.start {
                break;
            }
            let file_to_remove = files[right_most_len - 1].pop().unwrap();
            output += file_to_remove.id as u128
                * ((empty.start * file_to_remove.len) as u128
                    + (file_to_remove.len * file_to_remove.len - file_to_remove.len) as u128 / 2);
            empty.len -= file_to_remove.len;
            empty.start += file_to_remove.len;
        }
    }
    for len_rem in files {
        for file_rem in len_rem {
            output += file_rem.id as u128
                * ((file_rem.start * file_rem.len) as u128
                    + (file_rem.len * file_rem.len - file_rem.len) as u128 / 2);
        }
    }
    output
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
    fn test_2_f() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2_faster(&contents), 2858);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 2858);
    }
}
