fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '#' => {1},
                    '^' => {2},
                    _ => {0},
                })
                .collect::<Vec<i32>>()
        })
        .collect();
    return output1
}

use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0,0);
    let mut guard_been = HashSet::new();
    let mut obstacles = HashSet::new(); 
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == 1 {
                obstacles.insert((x,y));
            }
            if vec1[x][y] == 2 {
                guard_pos = (x,y);
                guard_been.insert(guard_pos);
            }
        }
    }
    let dirs = [(usize::MAX,0),(0,1),(1,0),(0,usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    loop {
        let mut next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1));
        if obstacles.contains(&next_pos) {
            cur_dir_idx = (cur_dir_idx + 1)%4;
            cur_dir = dirs[cur_dir_idx];
            next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1))
        }
        if next_pos.0 >= vec1.len() || next_pos.1 >= vec1[0].len() {
            break
        }
        guard_been.insert(next_pos);
        guard_pos = next_pos;
        
    }
    return guard_been.len() as i32
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i32 {
    let vec1 = parse(input);
    let mut guard_pos = (0,0);
    let mut guard_been = HashSet::new();
    let mut orig_obstacles = HashSet::new(); 
    let dirs = [(usize::MAX,0),(0,1),(1,0),(0,usize::MAX)];
    let mut cur_dir = dirs[0];
    let mut cur_dir_idx = 0;
    for x in 0..vec1.len() {
        for y in 0..vec1[0].len() {
            if vec1[x][y] == 1 {
                orig_obstacles.insert((x,y));
            }
            if vec1[x][y] == 2 {
                guard_pos = (x,y);
                guard_been.insert(guard_pos);
            }
        }
    }
    let guard_orig_start = guard_pos.clone();
    loop {
        let mut next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1));
        if orig_obstacles.contains(&next_pos) {
            cur_dir_idx = (cur_dir_idx + 1)%4;
            cur_dir = dirs[cur_dir_idx];
            next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1))
        }
        if next_pos.0 >= vec1.len() || next_pos.1 >= vec1[0].len() {
            break
        }
        guard_been.insert(next_pos);
        guard_pos = next_pos;
        
    }
    let mut nr_loops = 0;
    for extra_obst in guard_been {
        if extra_obst == guard_orig_start {
            continue;
        }
        let mut obstacles_new = orig_obstacles.clone();
        let mut guard_been_new_dir = HashSet::new();
        obstacles_new.insert((extra_obst.0,extra_obst.1));
        // println!("{:?}", extra_obst);
        guard_pos = guard_orig_start.clone();
        cur_dir_idx = 0;
        cur_dir = dirs[cur_dir_idx];
        guard_been_new_dir.insert((guard_pos, cur_dir_idx));
        loop {
            let mut next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1));
            if obstacles_new.contains(&next_pos) {
                cur_dir_idx = (cur_dir_idx + 1)%4;
                cur_dir = dirs[cur_dir_idx];
                next_pos = (guard_pos.0.wrapping_add(cur_dir.0), guard_pos.1.wrapping_add(cur_dir.1))
            }

            if next_pos.0 >= vec1.len() || next_pos.1 >= vec1[0].len() {
                break
            }
            if guard_been_new_dir.contains(&(next_pos, cur_dir_idx)) {
                println!("loop detected at {:?}", extra_obst);
                nr_loops += 1;
                break;
            }
            guard_been_new_dir.insert((next_pos, cur_dir_idx));
            guard_pos = next_pos;
            
        }
        // println!("{:?}", guard_been_new_dir.len())

    }
    return nr_loops
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
        assert_eq!(part1(&contents), 41);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 6);
    }
}