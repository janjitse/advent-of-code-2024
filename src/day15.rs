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
        let mut p = extra.chars().collect::<Vec<char>>();
        movement.append(&mut p);
    }

    // println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (map, movement)
}

use fxhash::{FxHashMap, FxHashSet};

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let (map, movement) = parse(input);
    // println!("{:?}", movement);
    let mut robot_location = (0, 0);
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
    let robot_move_vec = [
        ('v', (1, 0)),
        ('>', (0, 1)),
        ('^', (usize::MAX, 0)),
        ('<', (0, usize::MAX)),
    ];
    let robot_movement = FxHashMap::from_iter(robot_move_vec);
    for m in movement {
        let robot_dir = robot_movement[&m];
        // println!("Trying direction: {:?}", robot_dir);
        let new_robot_loc = (
            robot_location.0.wrapping_add(robot_dir.0),
            robot_location.1.wrapping_add(robot_dir.1),
        );
        // println!("New trial loc: {:?}", new_robot_loc);
        if obstacles.contains(&new_robot_loc) {
            // println!("Obstacle found at {:?}", new_robot_loc);
            continue;
        }
        if boxes.contains(&new_robot_loc) {
            // println!("Box found at {:?}", new_robot_loc);
            let mut move_possible = true;
            let mut box_moved = new_robot_loc;
            while boxes.contains(&box_moved) {
                box_moved = (
                    box_moved.0.wrapping_add(robot_dir.0),
                    box_moved.1.wrapping_add(robot_dir.1),
                );
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
                continue;
            }
        }
        // println!("Location updated to: {:?}", new_robot_loc);
        robot_location = new_robot_loc;
    }
    // print_map((map.len(), map[0].len()), &boxes, &obstacles, &robot_location, 1);

    boxes
        .into_iter()
        .map(|b| b.0 as u64 * 100 + b.1 as u64)
        .sum()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let (map, movement) = parse(input);
    let mut robot_location = (0, 0);
    let mut obstacles = FxHashSet::default();
    let mut left_boxes = FxHashSet::default();
    for (idx_row, row) in map.iter().enumerate() {
        for (idx_col, &col) in row.iter().enumerate() {
            if col == '@' {
                robot_location = (idx_row, 2 * idx_col);
            } else if col == '#' {
                obstacles.insert((idx_row, 2 * idx_col));
                obstacles.insert((idx_row, 2 * idx_col + 1));
            } else if col == 'O' {
                left_boxes.insert((idx_row, 2 * idx_col));
            }
        }
    }
    let robot_movement = FxHashMap::from_iter([
        ('v', (1, 0)),
        ('>', (0, 1)),
        ('<', (0, usize::MAX)),
        ('^', (usize::MAX, 0)),
    ]);
    for m in movement {
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
        let robot_dir = robot_movement[&m];
        // println!("Trying direction: {:?}", robot_dir);
        let new_robot_loc = (
            robot_location.0.wrapping_add(robot_dir.0),
            robot_location.1.wrapping_add(robot_dir.1),
        );
        // println!("New trial loc: {:?}", new_robot_loc);
        if obstacles.contains(&new_robot_loc) {
            // println!("Obstacle found at {:?}", new_robot_loc);
            continue;
        }
        if left_boxes.contains(&new_robot_loc)
            || left_boxes.contains(&(new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX)))
        {
            // println!("Box found at {:?}", new_robot_loc);
            let mut left_box_bloc = FxHashSet::default();
            let mut check = VecDeque::new();
            if left_boxes.contains(&new_robot_loc) {
                left_box_bloc.insert(new_robot_loc);
                check.push_back(new_robot_loc);
                check.push_back((new_robot_loc.0, new_robot_loc.1 + 1));
            }
            if left_boxes.contains(&(new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX))) {
                left_box_bloc.insert((new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX)));
                check.push_back((new_robot_loc.0, new_robot_loc.1.wrapping_add(usize::MAX)));
                check.push_back(new_robot_loc);
            }
            while let Some(b) = check.pop_front() {
                let b_new = (b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1));
                if left_boxes.contains(&b_new)
                    || left_boxes.contains(&(b_new.0, b_new.1.wrapping_add(usize::MAX)))
                {
                    if left_boxes.contains(&b_new) {
                        if left_box_bloc.insert(b_new) {
                            check.push_back(b_new);
                            check.push_back((b_new.0, b_new.1 + 1));
                        }
                    }
                    if left_boxes.contains(&(b_new.0, b_new.1.wrapping_add(usize::MAX))) {
                        if left_box_bloc.insert((b_new.0, b_new.1.wrapping_add(usize::MAX))) {
                            check.push_back(b_new);
                            check.push_back((b_new.0, b_new.1.wrapping_add(usize::MAX)));
                        }
                    }
                }
            }
            // println!("Moving block: {:?}, {:?}", left_box_bloc, right_box_bloc);
            let mut move_possible = true;
            for b in &left_box_bloc {
                if obstacles
                    .contains(&(b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1)))
                    || obstacles.contains(&(
                        b.0.wrapping_add(robot_dir.0),
                        b.1.wrapping_add(robot_dir.1) + 1,
                    ))
                {
                    move_possible = false;
                    break;
                }
            }
            if move_possible {
                // this has to be done sequentially, otherwise we might remove stuff we've added
                for b in left_box_bloc.iter() {
                    left_boxes.remove(b);
                }
                for b in left_box_bloc.iter() {
                    left_boxes
                        .insert((b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1)));
                }
            } else {
                continue;
            }
        }
        // println!("Location updated to: {:?}", new_robot_loc);
        robot_location = new_robot_loc;
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
    }
    // print_map((map.len(), map[0].len()), &left_boxes, &obstacles, &robot_location, 2);

    // println!("{:?}", map);
    left_boxes
        .into_iter()
        .map(|x| x.0 as u64 * 100 + x.1 as u64)
        .sum()
}

#[allow(dead_code)]
fn print_map(
    map_size: (usize, usize),
    left_boxes: &FxHashSet<(usize, usize)>,
    obstacles: &FxHashSet<(usize, usize)>,
    robot_loc: &(usize, usize),
    part: usize,
) {
    let mut new_map = vec![vec!['.'; part * map_size.1]; map_size.0];
    new_map[robot_loc.0][robot_loc.1] = '@';
    for b in left_boxes.iter() {
        if part == 2 {
            new_map[b.0][b.1] = '[';
            new_map[b.0][b.1 + 1] = ']';
        } else {
            new_map[b.0][b.1] = 'O';
        }
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

#[aoc(day15, part2, ids)]
fn part2_ids(input: &str) -> u64 {
    let (map, movement) = parse(input);
    let mut robot_location = (0, 0);
    let mut obstacles = FxHashSet::default();
    let mut boxes_loc = FxHashMap::default();
    let mut loc_box = FxHashMap::default();
    let mut box_id = 0;
    for (idx_row, row) in map.iter().enumerate() {
        for (idx_col, &col) in row.iter().enumerate() {
            if col == '@' {
                robot_location = (idx_row, 2 * idx_col);
            } else if col == '#' {
                obstacles.insert((idx_row, 2 * idx_col));
                obstacles.insert((idx_row, 2 * idx_col + 1));
            } else if col == 'O' {
                for w in [0, 1] {
                    loc_box
                        .entry(box_id)
                        .or_insert(vec![])
                        .push((idx_row, 2 * idx_col + w));
                    boxes_loc.insert((idx_row, 2 * idx_col + w), box_id);
                }
                box_id += 1;
            }
        }
    }
    let robot_movement = FxHashMap::from_iter([
        ('v', (1, 0)),
        ('>', (0, 1)),
        ('<', (0, usize::MAX)),
        ('^', (usize::MAX, 0)),
    ]);
    for m in movement {
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
        let robot_dir = robot_movement[&m];
        // println!("Trying direction: {:?}", robot_dir);
        let new_robot_loc = (
            robot_location.0.wrapping_add(robot_dir.0),
            robot_location.1.wrapping_add(robot_dir.1),
        );
        // println!("New trial loc: {:?}", new_robot_loc);
        if obstacles.contains(&new_robot_loc) {
            // println!("Obstacle found at {:?}", new_robot_loc);
            continue;
        }
        if let Some(box_id) = boxes_loc.get(&new_robot_loc) {
            let mut check = VecDeque::new();
            let mut box_bloc = FxHashSet::default();
            for &b in loc_box[box_id].iter() {
                box_bloc.insert(b);
                check.push_back(b);
            }
            while let Some(box_part) = check.pop_front() {
                let next_box = (
                    box_part.0.wrapping_add(robot_dir.0),
                    box_part.1.wrapping_add(robot_dir.1),
                );
                if let Some(next_box_id) = boxes_loc.get(&next_box) {
                    for &b in loc_box[next_box_id].iter() {
                        let new = box_bloc.insert(b);
                        if new {
                            check.push_back(b);
                        }
                    }
                }
            }
            let mut possible = true;
            for b in box_bloc.iter() {
                if obstacles
                    .contains(&(b.0.wrapping_add(robot_dir.0), b.1.wrapping_add(robot_dir.1)))
                {
                    possible = false;
                    break;
                }
            }
            // println!("Moving {:?}", box_bloc);
            if possible {
                // Get all box ids, and remove the locations from boxes_loc
                // Get location from loc_box, and update, and add to boxes_loc again
                let box_ids = FxHashSet::from_iter(
                    box_bloc.into_iter().map(|x| boxes_loc.remove(&x).unwrap()),
                );
                for b_i in box_ids {
                    let locs = loc_box.get_mut(&b_i).unwrap();
                    for loc_idx in 0..locs.len() {
                        let new_loc = (
                            locs[loc_idx].0.wrapping_add(robot_dir.0),
                            locs[loc_idx].1.wrapping_add(robot_dir.1),
                        );
                        locs[loc_idx] = new_loc;
                        boxes_loc.insert(new_loc, b_i);
                    }
                }
            } else {
                continue;
            }
        }
        // println!("Location updated to: {:?}", new_robot_loc);
        robot_location = new_robot_loc;
        // print_map(&map, &left_boxes, &right_boxes, &obstacles, &robot_location);
    }
    // print_map((map.len(), map[0].len()), &left_boxes, &right_boxes, &obstacles, &robot_location, 2);

    loc_box
        .values()
        .map(|x| {
            x.iter().min_by_key(|y| y.0).unwrap().0 as u64 * 100
                + x.iter().min_by_key(|y| y.1).unwrap().1 as u64
        })
        .sum()
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
        assert_eq!(part2(&contents), 9021);
    }

    #[test]
    fn test_2_tiny() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 618);
    }

    #[test]
    fn test_2_tiny_ids() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2_ids(&contents), 618);
    }
}
