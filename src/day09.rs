use std::iter;
use std::time::SystemTime;

fn parse(input: &str) -> Vec<usize> {
    let time_start = SystemTime::now();
    let output1 = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
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
        checksum_vec.extend(iter::repeat(to_push).take(*file_len));
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
            let end = start + *file_len;
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
            let end = start + *file_len;
            freespaces[*file_len - 1].push(File {
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

struct File2 {
    start: usize,
    id: usize,
}

struct Empty {
    start: usize,
    len: usize,
}

#[aoc(day9, part2, faster)]
fn part2_faster(input: &str) -> u128 {
    let x = parse(input);
    let mut files = [
        Vec::with_capacity(0),
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
    let mut last_non_empty = [0; 10];
    let mut right_most_start = [0; 10];
    let mut current_end = 0;
    let mut empty_spaces: Vec<Empty> = Vec::with_capacity(x.len() / 2);
    for (id, len) in x.chunks(2).enumerate() {
        files[len[0]].push(File2 {
            start: current_end,
            id,
        });
        right_most_start[len[0]] = current_end;
        current_end += len[0];
        let empty_len = *len.get(1).unwrap_or(&0);
        if empty_len > 0 {
            empty_spaces.push(Empty {
                start: current_end,
                len: empty_len,
            });
            current_end += empty_len;
        }
    }
    for l in 1..10 {
        last_non_empty[l] = files[l].len();
    }

    let mut output = 0;
    for mut empty in empty_spaces {
        if empty.start > *right_most_start.iter().max().unwrap_or(&0) {
            break;
        }
        while empty.len > 0 {
            let mut right_most_len = 0;
            let mut right_most = empty.start;
            for (file_len, right_most_start_len) in
                right_most_start.iter().enumerate().skip(1).take(empty.len)
            {
                if *right_most_start_len > right_most {
                    right_most = *right_most_start_len;
                    right_most_len = file_len;
                }
            }
            if right_most_len == 0 {
                break;
            }

            last_non_empty[right_most_len] -= 1;
            output += files[right_most_len][last_non_empty[right_most_len]].id as u128
                * ((empty.start * right_most_len) as u128
                    + (right_most_len * right_most_len - right_most_len) as u128 / 2);

            right_most_start[right_most_len] =
                files[right_most_len][last_non_empty[right_most_len] - 1].start;
            empty.len -= right_most_len;
            empty.start += right_most_len;
        }
    }

    for (len, len_rem) in files.into_iter().enumerate().skip(1) {
        let len_contrib = ((len * len - len) / 2) as u128;
        for file in len_rem.iter().take(last_non_empty[len]) {
            output += file.id as u128 * ((file.start * len) as u128 + len_contrib);
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
