use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<char>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use fxhash::{FxHashSet, FxHashMap};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    loc: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
struct PriorityElement {
    dist: u64,
    pos: Position,
    nr_cheats: u64,
}

impl Ord for PriorityElement {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.dist).cmp(&Reverse(other.dist))
    }
}

impl PartialOrd for PriorityElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Reverse(self.dist).cmp(&Reverse(other.dist)))
    }
}

#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut spaces = FxHashSet::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut obstacles = FxHashSet::default();
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '.' || col == 'S' || col == 'E' {
                spaces.insert((row_idx, col_idx));
            }
            if col == 'E' {
                end = (row_idx, col_idx);
            }
            if col == 'S' {
                start = (row_idx, col_idx);
            }
            if col == '#' {
                obstacles.insert((row_idx, col_idx));
            }
        }
    }

    let clockwise = FxHashMap::from_iter([
        ((0, 1), (1, 0)),
        ((1, 0), (0, usize::MAX)),
        ((0, usize::MAX), (usize::MAX, 0)),
        ((usize::MAX, 0), (0, 1)),
    ]);
    let mut cheating_paths = 0;
    let len_to_visit = spaces.len() - 100;
    for (cheat_idx, cheat) in obstacles.iter().enumerate() {
        // println!("{:?}", cheat_idx);
        let starting_pos = Position {
            loc: start,
        };
        spaces.insert(*cheat);
        let mut p_q = BinaryHeap::new();
        let mut visited = FxHashSet::default();
        p_q.push(PriorityElement {
            dist: 0,
            pos: starting_pos,
            nr_cheats: 0,
        });
        let mut cheating_length = u64::MAX;
        while let Some(PriorityElement {
            dist: d,
            pos,
            nr_cheats: 0,
        }) = p_q.pop()
        {
            if d as usize > len_to_visit {
                break;
            }
            if pos.loc == end {
                cheating_length  = d + 1;
                break;
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos.clone());
            let mut cur_dir = (0,1);
            for _ in 0..4 {
                let next_pos = (
                    pos.loc.0.wrapping_add(cur_dir.0),
                    pos.loc.1.wrapping_add(cur_dir.1),
                );
                if spaces.contains(&next_pos) {
                    let xy = Position {
                        loc: next_pos,
                    };
                    if !visited.contains(&xy) {
                        p_q.push(PriorityElement {
                            dist: d + 1,
                            pos: xy,
                            nr_cheats: 0,
                        });
                    }
                }
                cur_dir = clockwise[&cur_dir];
            }
        }
        if cheating_length <= len_to_visit as u64 {
            cheating_paths += 1;
        }
        spaces.remove(cheat);
    }

    println!("{:?}", spaces.len());
    println!("{:?}", len_to_visit);
    cheating_paths
}

#[aoc(day20, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let mut spaces = FxHashSet::default();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut obstacles = FxHashSet::default();
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &col) in row.iter().enumerate() {
            if col == '.' || col == 'S' || col == 'E' {
                spaces.insert((row_idx, col_idx));
            }
            if col == 'E' {
                end = (row_idx, col_idx);
            }
            if col == 'S' {
                start = (row_idx, col_idx);
            }
            if col == '#' {
                obstacles.insert((row_idx, col_idx));
            }
        }
    }

    let directions = vec![(0,1),(1,0),(0,usize::MAX),(usize::MAX,0)];
    let mut p_q = BinaryHeap::new();
    let mut distance_ord = FxHashMap::default();
    let starting_pos = Position {
        loc: start,
    };
    p_q.push(PriorityElement {
        dist: 0,
        pos: starting_pos,
        nr_cheats: 0,
    });
    let mut cheating_length = u64::MAX;
    while let Some(PriorityElement {
        dist: d,
        pos,
        nr_cheats: 0,
    }) = p_q.pop()
    {
        if pos.loc == end {
            distance_ord.insert(pos.loc.clone(), d as i64);
            break;
        }
        if distance_ord.contains_key(&pos.loc) {
            continue;
        }
        distance_ord.insert(pos.loc.clone(), d as i64);
        let mut cur_dir = (0,1);
        for cur_dir in directions.iter() {
            let next_pos = (
                pos.loc.0.wrapping_add(cur_dir.0),
                pos.loc.1.wrapping_add(cur_dir.1),
            );
            if spaces.contains(&next_pos) {
                let xy = Position {
                    loc: next_pos,
                };
                if !distance_ord.contains_key(&xy.loc) {
                    p_q.push(PriorityElement {
                        dist: d + 1,
                        pos: xy,
                        nr_cheats: 0,
                    });
                }
            }
        }
    }    

    let mut cheating_paths = 0;
    // println!("{:?}", distance_ord);
    // let len_to_visit = spaces.len() - 3;
    let full_distance = *distance_ord.get(&end).unwrap();
    let mut cheat_start_end = FxHashSet::default();
    for (cheat_idx, cheat_start) in spaces.iter().enumerate() {
        // println!("{:?}", cheat_idx);
        
        let mut cheat_start_loc = (0,0);
        // for dir in directions.iter() {
            // let start_loc = (cheat_start.0.wrapping_add(dir.0), cheat_start.1.wrapping_add(dir.1));
            let start_loc = (cheat_start.0, cheat_start.1);
            let cheat_length_start = *distance_ord.get(&start_loc).unwrap_or(&i64::MAX);
            cheat_start_loc = start_loc;
            
        
            if cheat_length_start == i64::MAX {
                continue;
            }
            for &cheat_end in spaces.iter() {
                // for dir in directions.iter() {
                    let cheat_end_loc = (cheat_end.0, cheat_end.1);
                    
                    let cheat_end_length = *distance_ord.get(&cheat_end_loc).unwrap_or(&i64::MAX);
                    if cheat_end_length == i64::MAX {
                        continue;
                    }
                    if cheat_start_end.contains(&(cheat_start_loc, cheat_end_loc)) {
                        continue;
                    }
                    let cheat_length = cheat_start.0.abs_diff(cheat_end_loc.0) as i64 +  cheat_start.1.abs_diff(cheat_end_loc.1) as i64;
                    if cheat_length <= 20 {
                        // println!("{:?}, {:?}, {:?}", cheat_start, cheat_end, cheat_length);
                        if cheat_length_start + (full_distance - cheat_end_length) + cheat_length <= full_distance as i64 - 100 {
                            // println!("start at {:?}, {:?}, end at {:?}, {:?}, distance start: {:?}, distance_end: {:?}, cheat_length {:?}",  cheat_start_loc, cheat_start, cheat_end, cheat_end_loc, cheat_length_start,  cheat_end_length, cheat_length);
                            cheating_paths += 1;
                            cheat_start_end.insert((cheat_start_loc, cheat_end_loc));
                        }
                    }
                
            // }
        }

        
    }

    println!("{:?}", spaces.len());
    // println!("{:?}", len_to_visit);
    cheating_paths
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
        assert_eq!(part1(&contents), 0);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 0);
    }
}