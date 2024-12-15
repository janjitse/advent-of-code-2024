use std::{collections::VecDeque, time::SystemTime};

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let time_start = SystemTime::now();

    let mut lines = input.lines();
    let map = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut movement = vec![];

    for extra in lines {
        let  mut p = extra.chars().collect::<Vec<char>>();
        movement.append(&mut p);

    }


    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    return (map, movement);
}

use fxhash::{FxHashSet, FxHashMap};

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let (map, movement) = parse(input);
    // println!("{:?}", movement);
    let mut robot_location = (0,0);
    let mut obstacles = FxHashSet::default();
    let mut boxes = FxHashSet::default();
    for (idx_row, row) in map.iter().enumerate() {
        for (idx_col, &col) in row.iter().enumerate() {
            if col == '@' {
                robot_location = (idx_row, idx_col);
            } else if col == '#' {
                obstacles.insert((idx_row, idx_col));
            } else if col == 'O' {
                boxes.insert((idx_row, idx_col));
            }
        }
    }
    // println!("{:?}", boxes);
    let robot_move_vec = [('v', (1,0)),('>', (0,1)), ('<', (0,usize::MAX)), ('^', (usize::MAX,0))];
    let robot_movement = FxHashMap::from_iter(robot_move_vec);
    for m in movement {
        let robot_dir = robot_movement[&m];
        // println!("Trying direction: {:?}", robot_dir);
        let new_robot_loc = (robot_location.0.wrapping_add(robot_dir.0), robot_location.1.wrapping_add(robot_dir.1));
        // println!("New trial loc: {:?}", new_robot_loc);
        if obstacles.contains(&new_robot_loc) {
            // println!("Obstacle found at {:?}", new_robot_loc);
            continue;
        }
        if boxes.contains(&new_robot_loc) {
            // println!("Box found at {:?}", new_robot_loc);
            let mut move_possible = true;
            let mut box_moved = new_robot_loc.clone();
            while boxes.contains(&box_moved) {
                box_moved = (box_moved.0.wrapping_add(robot_dir.0), box_moved.1.wrapping_add(robot_dir.1));
                if obstacles.contains(&box_moved) {
                    // println!("Obstacle while moving box at {:?}", box_moved);
                    move_possible = false;
                    break;
                }

            }
            if move_possible {
                boxes.remove(&new_robot_loc);
                boxes.insert(box_moved);
            } else {
                continue
            }
        }
        // println!("Location updated to: {:?}", new_robot_loc);
        robot_location = new_robot_loc;
        
    }
    // let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];
    // for b in boxes.iter() {
    //     new_map[b.0][b.1] = 'O';
    // }
    // for l in new_map.iter() {
    //     println!("{:?}", l.iter().collect::<String>());
    // }
    // for o in obstacles {
    //     new_map[o.0][o.1] = '#';
    // }
    // for l in new_map.iter() {
    //     println!("{:?}", l.iter().collect::<String>());
    // }
    // println!("{:?}", map);
    let mut score = 0;
    for b in boxes {
        score += b.0 as u64 * 100 + b.1 as u64;
    }
    return score;
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let (map, movement) = parse(input);
    let mut robot_location = (0,0);
    let mut obstacles = FxHashSet::default();
    let mut left_boxes = FxHashSet::default();
    let mut right_boxes = FxHashSet::default();
    for (idx_row, row) in map.iter().enumerate() {
        for (idx_col, &col) in row.iter().enumerate() {
            if col == '@' {
                robot_location = (idx_row, 2*idx_col);
            } else if col == '#' {
                obstacles.insert((idx_row, 2*idx_col));
                obstacles.insert((idx_row, 2*idx_col + 1));
            } else if col == 'O' {
                left_boxes.insert((idx_row, 2* idx_col));
                right_boxes.insert((idx_row, 2* idx_col + 1));
            }
        }
    }
    let robot_move_vec = [('v', (1,0)),('>', (0,1)), ('<', (0,usize::MAX)), ('^', (usize::MAX,0))];
    let robot_movement = FxHashMap::from_iter(robot_move_vec);
    for m in movement {
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
        let robot_dir = robot_movement[&m];
        // println!("Trying direction: {:?}", robot_dir);
        let new_robot_loc = (robot_location.0.wrapping_add(robot_dir.0), robot_location.1.wrapping_add(robot_dir.1));
        // println!("New trial loc: {:?}", new_robot_loc);
        if obstacles.contains(&new_robot_loc) {
            // println!("Obstacle found at {:?}", new_robot_loc);
            continue;
        }
        if left_boxes.contains(&new_robot_loc) || left_boxes.contains(&(new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX))) {
            // println!("Box found at {:?}", new_robot_loc);
            let mut move_possible = true;
            // let mut box_moved = new_robot_loc.clone();
            let mut left_box_bloc = FxHashSet::default();
            let mut right_box_bloc = FxHashSet::default();
            let mut check = VecDeque::new();
            if left_boxes.contains(&new_robot_loc) {
                left_box_bloc.insert(new_robot_loc);
                check.push_back(new_robot_loc);
                right_box_bloc.insert((new_robot_loc.0, new_robot_loc.1 + 1));
                check.push_back((new_robot_loc.0, new_robot_loc.1 + 1));
            }
            if left_boxes.contains(&(new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX))) {
                right_box_bloc.insert(new_robot_loc);
                check.push_back(new_robot_loc);
                left_box_bloc.insert((new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX)));
                check.push_back((new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX)));
            }
            while let Some(b) = check.pop_front() {
                let b_new = (b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1));
                if left_boxes.contains(&b_new) || left_boxes.contains(&(b_new.0, b_new.1.wrapping_add(usize::MAX))) {
                    if left_boxes.contains(&b_new) {
                        let mut new = left_box_bloc.insert(b_new);
                        if new {
                            check.push_back(b_new);
                        }
                        
                        new = right_box_bloc.insert((b_new.0, b_new.1 + 1));
                        if new {
                            check.push_back((b_new.0, b_new.1 + 1));
                        }
                        
                    }
                    if left_boxes.contains(&(b_new.0, b_new.1.wrapping_add(usize::MAX))) {
                        let mut new = right_box_bloc.insert(b_new);
                        if new {
                            check.push_back(b_new);
                        }
                        new = left_box_bloc.insert((b_new.0, b_new.1.wrapping_add(usize::MAX)));
                        if new {
                            check.push_back((b_new.0, b_new.1.wrapping_add(usize::MAX)));
                        }
                    }
                }
            }
            // println!("Moving block: {:?}, {:?}", left_box_bloc, right_box_bloc);
            let mut move_possible = true;
            for b in left_box_bloc.iter() {
                if obstacles.contains(&(b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1))) {
                    move_possible = false;
                    break;
                }
            }
            for b in right_box_bloc.iter() {
                if obstacles.contains(&(b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1))) {
                    move_possible = false;
                    break;
                }
            }
            if move_possible {
                for b in left_box_bloc.iter() {
                    left_boxes.remove(b);
                }
                for b in left_box_bloc.iter() {
                    left_boxes.insert((b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1)));
                }
                for b in right_box_bloc.iter() {
                    right_boxes.remove(b);
                }
                for b in right_box_bloc.iter() {
                    right_boxes.insert((b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1)));
                }


            } else {
                continue;
            }

            
        }
        // println!("Location updated to: {:?}", new_robot_loc);
        robot_location = new_robot_loc;
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
    }
    // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);

    // println!("{:?}", map);
    let mut score = 0;

    let width = 2 * map[0].len();
    let height = map.len();
    for b in left_boxes {
        let mut min_dist_y = usize::MAX;
        min_dist_y = min_dist_y.min(b.0);
        // min_dist_y = min_dist_y.min(height - b.0);
        let mut min_dist_x = usize::MAX;
        min_dist_x = min_dist_x.min(b.1);
        // min_dist_x = min_dist_x.min(width - b.1 - 1);
        score += 100 * min_dist_y as u64 + min_dist_x as u64; 
    }
    return score;
}


fn print_map(map: &Vec<Vec<char>>, left_boxes: &FxHashSet<(usize, usize)>, right_boxes: &FxHashSet<(usize, usize)>, obstacles: &FxHashSet<(usize, usize)>, robot_loc: &(usize,usize)) {
    let mut new_map = vec![vec!['.'; 2*map[0].len()]; map.len()];
    new_map[robot_loc.0][robot_loc.1] = '@';
    for b in left_boxes.iter() {
        new_map[b.0][b.1] = '[';
    }
    for b in right_boxes.iter() {
        new_map[b.0][b.1] = ']';
    }
    for l in new_map.iter() {
        println!("{:?}", l.iter().collect::<String>());
    }
    for o in obstacles {
        new_map[o.0][o.1] = '#';
    }
    for l in new_map.iter() {
        println!("{:?}", l.iter().collect::<String>());
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
        assert_eq!(part1(&contents), 10092);
    }

    #[test]
    fn test_1_tiny() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 104);
    }


    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents),  9021);
    }

    #[test]
    fn test_2_tiny() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 9021);
    }
}