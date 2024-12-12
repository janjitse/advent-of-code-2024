use std::{collections::{HashSet, VecDeque}, time::SystemTime};

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


use fxhash::FxHashSet;

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let mut done: FxHashSet<(usize,usize)> = FxHashSet::default();
    let mut regions = vec![];
    loop {
        let mut current_char = '_';
        let mut current_region: FxHashSet<(usize,usize)> = FxHashSet::default();
        for (row_idx, row) in x.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if done.contains(&(row_idx, col_idx)) {
                    continue;
                }
                else if current_char == '_' {
                    current_char = c;
                    current_region = dfs(&x, current_char, (row_idx, col_idx));
                    // done = done.union(&current_region.iter().map(|x| *x)).collect();
                    for d in current_region.clone().iter() {
                        done.insert(*d);
                    }
                }

            }
        }
        if current_char == '_' {
            break;
        }
        regions.push(current_region);

    }
    let mut output = 0;
    for region in regions {
        let mut area = 0;
        let mut perimeter = 0;
        for c in region.iter() {
            area += 1;
            for dir in [(0,1),(1,0),(0,usize::MAX),(usize::MAX,0)] {
                if !region.contains(&(c.0.wrapping_add(dir.0), c.1.wrapping_add(dir.1))) {
                    perimeter += 1;
                }
            }
        }

        output += area * perimeter;
    }

    output
}



fn dfs(x: &Vec<Vec<char>>, cur_char: char, start_loc: (usize,usize)) -> FxHashSet<(usize,usize)> {
    let mut region = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(start_loc);
    region.insert(start_loc);
    while let Some(next_c) = queue.pop_front() {
        for dir in [(0,1),(1,0),(usize::MAX,0),(0,usize::MAX)] {
            let next_loc = (next_c.0.wrapping_add(dir.0), next_c.1.wrapping_add(dir.1));
            if next_loc.0 < x.len() && next_loc.1 < x[0].len() && x[next_loc.0][next_loc.1] == cur_char {
                if !region.contains(&next_loc) {
                    queue.push_back(next_loc);
                    region.insert(next_loc);
                }
            }
        }
    }
    return region;
}
#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let x = parse(input);
    let mut done: FxHashSet<(usize,usize)> = FxHashSet::default();
    let mut regions = vec![];
    loop {
        let mut current_char = '_';
        let mut current_region: FxHashSet<(usize,usize)> = FxHashSet::default();
        for (row_idx, row) in x.iter().enumerate() {
            for (col_idx, &c) in row.iter().enumerate() {
                if done.contains(&(row_idx, col_idx)) {
                    continue;
                }
                else if current_char == '_' {
                    current_char = c;
                    current_region = dfs(&x, current_char, (row_idx, col_idx));
                    // done = done.union(&current_region.iter().map(|x| *x)).collect();
                    for d in current_region.clone().iter() {
                        done.insert(*d);
                    }
                }

            }
        }
        if current_char == '_' {
            break;
        }
        regions.push(current_region);

    }
    let mut output = 0;
    for region in regions {
        let mut area = 0;
        let mut corners = 0;
        let mut inner_corners = 0;
        // let mut inner_corners = 0;
        for c in region.iter() {
            area += 1;
            // let mut edge = vec![0, 0, 0, 0];
            // for (idx, dir) in [(0,1),(1,0),(0,usize::MAX),(usize::MAX,0)].into_iter().enumerate() {
            //     if !region.contains(&(c.0.wrapping_add(dir.0), c.1.wrapping_add(dir.1))) {
            //         // perimeter += 1;
            //         edge[idx] = 1;
            //     }
            // }
            // //This counts outer corners
            // if edge[0] == 1 && edge[1] == 1 {
            //     outer_corners += 2;
            // }
            // if edge[1] == 1 && edge[2] == 1 {
            //     outer_corners += 2;
            // }
            // if edge[2] == 1 && edge[3] == 1 {
            //     outer_corners += 2;
            // }
            // if edge[3] == 1 && edge[0] == 1 {
            //     outer_corners += 2;
            // }
            let mut conv = vec![vec![0,0,0],vec![0,0,0], vec![0,0,0]];
            // conv[1][1] = 1
            for dir in [(0,1),(1,1),(1,0),(usize::MAX,0),(0,usize::MAX),(1,usize::MAX),(usize::MAX,usize::MAX),(usize::MAX,1)] {
                if !region.contains(&(c.0.wrapping_add(dir.0), c.1.wrapping_add(dir.1))) {
                    // perimeter += 1;
                    conv[1usize.wrapping_add(dir.0)][1usize.wrapping_add(dir.1)] = 1;
                }
            }

            //outer corners
            if conv[0][1] == 1 && conv[1][2] == 1 {
                corners += 2;
                inner_corners +=2;
            }
            if conv[1][2] == 1 && conv[2][1] == 1 {
                corners += 2;
                inner_corners +=2;
            }
            if conv[2][1] == 1 && conv[1][0] == 1 {
                corners += 2;
                inner_corners +=2;
            }
            if conv[1][0] == 1 && conv[0][1] == 1 {
                corners += 2;
                inner_corners +=2;
            }
            //inner corners
            if conv[0][1] == 0 && conv[1][2] == 0 && conv[0][2] == 1 {
                corners += 2;
            }
            if conv[1][2] == 0 && conv[2][1] == 0 && conv[2][2] == 1 {
                corners += 2;
            }
            if conv[2][1] == 0 && conv[1][0] == 0 && conv[2][0] == 1 {
                corners += 2;
            }
            if conv[1][0] == 0 && conv[0][1] == 0 && conv[0][0] == 1 {
                corners += 2;
            }
            
        }
        // println!("{:?}, {:?}, {:?}", area, inner_corners, corners-inner_corners);
        output += area * corners/2;
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
        assert_eq!(part1(&contents), 140);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 1206);
    }

    #[test]
    fn test_2a() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 368);
    }
}