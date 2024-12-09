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
fn part1(input: &str) -> u64 {
    let x = parse(input);
    // let total: u32 = x.iter().sum();
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
    println!("{:?}", checksum_vec.len());
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
    // println!("compacted: {:?}", compacted_vec);
    let mut output = 0;
    for (idx, val) in compacted_vec.into_iter().enumerate() {
        output += val as u64 * idx as u64
    }
    output
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    start: usize,
    end: usize,
    id: isize,
}

#[aoc(day9, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut files: Vec<File> = vec![];
    let mut freespace: Vec<File> = vec![];
    for (idx, file_len) in x.iter().enumerate() {
        if idx % 2 == 0 {
            let start = files
                .last()
                .unwrap_or(&File {
                    start: 0,
                    end: 0,
                    id: 0,
                })
                .end
                .max(
                    freespace
                        .last()
                        .unwrap_or(&File {
                            start: 0,
                            end: 0,
                            id: 0,
                        })
                        .end,
                );
            let end = start + *file_len as usize;
            files.push(File {
                start,
                end,
                id: (idx / 2) as isize,
            });
        } else {
            if *file_len == 0 {
                continue;
            }
            let start = files
                .last()
                .unwrap_or(&File {
                    start: 0,
                    end: 0,
                    id: 0,
                })
                .end
                .max(
                    freespace
                        .last()
                        .unwrap_or(&File {
                            start: 0,
                            end: 0,
                            id: 0,
                        })
                        .end,
                );
            let end = start + *file_len as usize;
            freespace.push(File { start, end, id: -1 });
        }
    }
    let mut compacted_files: Vec<File> = vec![];
    while let Some(next_file) = files.pop() {
        match find_next_free(next_file.end - next_file.start, &freespace, next_file.start) {
            None => {
                compacted_files.push(next_file);
            }
            Some(idx) => {
                let new_start = freespace[idx].start;
                let new_end = freespace[idx].start + next_file.end - next_file.start;
                let new_file = File {
                    start: new_start,
                    end: new_end,
                    id: next_file.id,
                };
                compacted_files.push(new_file);
                if new_end < freespace[idx].end {
                    freespace[idx].start = new_end;
                } else {
                    freespace.remove(idx);
                }
                let free_idx = freespace.binary_search(&next_file).unwrap_err();
                freespace.insert(free_idx, next_file);
                compact_around(free_idx, &mut freespace);
            }
        }
    }
    let mut checksum = 0;
    for c in compacted_files {
        for idx in c.start..c.end {
            checksum += idx as u64 * c.id as u64;
        }
    }
    checksum
}

fn find_next_free(file_len: usize, freespace_vec: &[File], max_start: usize) -> Option<usize> {
    for (idx, freespace) in freespace_vec.iter().enumerate() {
        if freespace.start > max_start {
            return None;
        }
        if freespace.end - freespace.start >= file_len {
            return Some(idx);
        }
    }
    None
}

fn compact_around(idx: usize, freespace_vec: &mut Vec<File>) {
    if idx < freespace_vec.len() - 1 && freespace_vec[idx + 1].start == freespace_vec[idx].end {
        freespace_vec[idx].end = freespace_vec[idx + 1].end;
        freespace_vec.remove(idx + 1);
    }
    if idx > 0 && freespace_vec[idx - 1].end == freespace_vec[idx].start {
        freespace_vec[idx - 1].end = freespace_vec[idx].end;
        freespace_vec.remove(idx);
    }
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
