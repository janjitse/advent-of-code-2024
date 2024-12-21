use std::{collections::VecDeque, time::SystemTime};

fn parse(input: &str) ->  Vec<String> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output2
}

use rustc_hash::{FxHashMap, FxHashSet};

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    let instructions = parse(input);
    let mapping_1 = all_paths_1();
    let mapping_2 = all_paths_2();
    
    println!("{:?}", mapping_1.keys().len());

    let mut outcome = 0;
    // println!("{:?}", mapping_2.keys());
    for code in instructions {
        let mut remaining = vec!['A'];
        let code_vec: Vec<char> = code.chars().collect();
        remaining.extend(code_vec);
        let remaining_vec_deque = VecDeque::from_iter(remaining.windows(2).map(|x| (x[0],x[1])));
        let min_vectors1 = recurse_1(remaining_vec_deque, &mapping_1);
        let mut min_vectors2: Vec<Vec<char>> = vec![];
        let mut min_length2 = usize::MAX;
        for k in min_vectors1 {
            let mut remaining2 = vec!['A'];
            remaining2.extend(k);
            let remaining_vec_deque2 = VecDeque::from_iter(remaining2.windows(2).map(|x| (x[0],x[1])));
            let results2 = recurse_1(remaining_vec_deque2, &mapping_2);
            for l in results2 {
                if l.len() < min_length2 {
                    min_length2 = l.len();
                    min_vectors2 = vec![l];
                } else if l.len() == min_length2 {
                    min_vectors2.push(l)
                }
            }
        }
        println!("{:?}", min_vectors2.len());

        let mut min_vectors3: Vec<Vec<char>> = vec![];
        let mut min_length3 = usize::MAX;
        for k in min_vectors2 {
            let mut remaining3 = vec!['A'];
            remaining3.extend(k);
            let remaining_vec_deque3 = VecDeque::from_iter(remaining3.windows(2).map(|x| (x[0],x[1])));
            let results3 = recurse_1(remaining_vec_deque3, &mapping_2);
            for l in results3 {
                if l.len() < min_length3 {
                    min_length3 = l.len();
                    min_vectors3 = vec![l];
                } else if l.len() == min_length2 {
                    min_vectors3.push(l)
                }
            }
        }
        println!("{:?}, {:?}", code, min_vectors3.first().unwrap().len());
        let code_digit: u64 = code[0..3].parse().unwrap();
        outcome += min_vectors3.first().unwrap().len() as u64 * code_digit;

    }
    outcome
}

fn recurse_1(mut remaining: VecDeque<(char,char)>, map1: &FxHashMap<(char,char), Vec<String>>) -> Vec<Vec<char>> {
    if remaining.is_empty() {
        return vec![vec![]];
    }
    let mut min_cost = u64::MAX;
    let mut return_value = vec![];
    let next_option = remaining.pop_front().unwrap();
    // println!("{:?}", next_option);
    let outcomes = recurse_1(remaining.clone(),map1);
    for option in map1.get(&next_option).unwrap() {
        let mut option_vec: Vec<char> = option.chars().collect();
        option_vec.push('A');
        
        for outcome in outcomes.iter() {
            let mut option_vec2 = option_vec.clone();
            option_vec2.extend(outcome);
            if (option_vec2.len() as u64) < min_cost {
                min_cost = option_vec2.len() as u64;
                return_value = vec![option_vec2];
            } else if (option_vec2.len() as u64) == min_cost {
                return_value.push(option_vec2);
            }    
        }

    }

    return_value    
}


fn recurse_2(mut remaining: VecDeque<(char,char)>, map1: &FxHashMap<(char,char), Vec<String>>) -> Vec<Vec<char>> {
    if remaining.is_empty() {
        return vec![vec![]];
    }
    // let mut min_cost = u64::MAX;
    // let mut return_value = vec![];
    let next_option = remaining.pop_front().unwrap();
    // println!("{:?}", next_option);
    let outcome = recurse_2(remaining.clone(),map1).first().unwrap().clone();
    // println!("Length outcomes {:?}", outcomes.len());
    let mut option_vec = map1.get(&next_option).unwrap().first().unwrap().chars().collect::<Vec<char>>();
    // for option in map1.get(&next_option).unwrap() {
        // let mut option_vec: Vec<char> = option.chars().collect();
        option_vec.push('A');
        
        // for outcome in outcomes.iter() {
            // let mut option_vec2 = option_vec.clone();
            option_vec.extend(outcome);
            // if (option_vec.len() as u64) < min_cost {
                // min_cost = option_vec.len() as u64;
                let return_value = vec![option_vec];
            // } else if (option_vec2.len() as u64) == min_cost {
            //     return_value.push(option_vec2);
            // }    
        // }

    // }

    return_value    
}


fn recurse_1_cost(mut remaining: VecDeque<(char,char)>, map1: &FxHashMap<(char,char), Vec<String>>, 
    cache: &mut FxHashMap<VecDeque<(char,char)>, Vec<Vec<char>>>
    ) -> Vec<Vec<char>> {
    if remaining.is_empty() {
        return vec![vec![]];
    }
    if cache.contains_key(&remaining) {
        return cache.get(&remaining).unwrap().clone()
    }
    let mut min_cost = u64::MAX;
    let mut return_value = vec![];
    let next_option = remaining.pop_front().unwrap();
    // println!("{:?}", next_option);
    let outcomes = recurse_1_cost(remaining.clone(), map1, cache);
    for option in map1.get(&next_option).unwrap() {
        let mut option_vec: Vec<char> = option.chars().collect();
        option_vec.push('A');
        
        for outcome in outcomes.iter() {
            let mut option_vec2 = option_vec.clone();
            option_vec2.extend(outcome);
            if (option_vec2.len() as u64) < min_cost {
                min_cost = option_vec2.len() as u64;
                return_value = vec![option_vec2];
            } else if (option_vec2.len() as u64) == min_cost {
                return_value.push(option_vec2);
            }    
        }

    }
    cache.insert(remaining, return_value.clone());

    return_value    
}


fn vector_cost(input: &Vec<char>, mapping_2: &FxHashMap<(char,char), Vec<String>>) -> usize {
    let mut cost = 0;
    let mut cost_vector = vec!['A'];
    cost_vector.extend(input);
    for p in cost_vector.windows(2) {
        cost += mapping_2.get(&(p[0], p[1])).unwrap().first().unwrap().len() + 1;
    }
    cost
}

fn all_paths_1() -> FxHashMap<(char,char), Vec<String>> {
    let open_vec = vec![(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2),(3,1),(3,2)];
    let mapping_vec = vec![
        ('A',(3,2)),
        ('0',(3,1)),
        ('1',(2,0)),
        ('2',(2,1)),
        ('3',(2,2)),
        ('4',(1,0)),
        ('5',(1,1)),
        ('6',(1,2)),
        ('7',(0,0)),
        ('8',(0,1)),
        ('9',(0,2))
    ];
    let directions = [(0,1),(1,0),(0,-1),(-1,0)];
    let mapping = FxHashMap::from_iter(mapping_vec);
    let mut char_paths = FxHashMap::default();
    let buttons = ['0','1','2','3','4','5','6','7','8','9','A'];
    let positions = FxHashSet::from_iter(open_vec);
    
    
    // for char_1 in buttons.iter() {
    for char_1 in buttons.iter() {
        let origin = *mapping.get(&char_1).unwrap();
        for char_2 in buttons.iter() {
            let end = *mapping.get(&char_2).unwrap();
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0),origin,vec![]));
            let mut all_paths = FxHashSet::default();
            let mut visited_distance = FxHashMap::default();
            let mut end_distance = u64::MAX;
            while let Some((distance, pos, history)) = queue.pop() {
                if distance.0 > end_distance {
                    break;
                }
                if distance.0 > *visited_distance.get(&pos).unwrap_or(&u64::MAX) {
                    continue;
                }
                if pos == end {
                    all_paths.insert(history);
                    end_distance = end_distance.min(distance.0);
                    continue;
                }
                visited_distance.insert(pos, distance.0);
                for dir in directions {
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if positions.contains(&next_pos) {
                        if distance.0 + 1 <= *visited_distance.get(&next_pos).unwrap_or(&u64::MAX) {
                            let mut new_history = history.clone();
                            new_history.push(dir);
                            let mut extra_distance = 1;
                            if *history.last().unwrap_or(&dir) != dir {
                                extra_distance += 100;
                            }
                            queue.push((Reverse(distance.0+extra_distance), next_pos, new_history));
                        }
                    }
                }
            }
            let mut shortest_len = usize::MAX;
            for p in all_paths.iter() {
                shortest_len = shortest_len.min(p.len());
            }
            for path in all_paths {
                if path.len() > shortest_len {
                    continue;
                }
                let mut path_chars = vec![];
                for step in path {
                    let char_step = match step {
                        (0,1) => '>',
                        (1,0) => 'v',
                        (-1,0) => '^',
                        (0,-1) => '<',
                        _ => panic!(),
                    };
                    path_chars.push(char_step);

                }
                let path_string: String = path_chars.into_iter().collect();
                char_paths.entry((*char_1,*char_2)).or_insert(vec![]).push(path_string);
            }            
        }
    }
    char_paths
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn all_paths_2() -> FxHashMap<(char,char), Vec<String>> {
    let open_vec = vec![(0,1),(0,2),(1,2),(1,1),(1,0)];
    let mapping_vec = vec![
        ('A',(0,2)),
        ('^',(0,1)),
        ('>',(1,2)),
        ('v',(1,1)),
        ('<',(1,0)),
    ];
    let directions = [(0,1),(1,0),(0,-1),(-1,0)];
    let mapping = FxHashMap::from_iter(mapping_vec);
    let mut char_paths = FxHashMap::default();
    let buttons = ['^','v','>','<','A'];
    let positions = FxHashSet::from_iter(open_vec);
    
    
    for char_1 in buttons.iter() {
        let origin = *mapping.get(&char_1).unwrap();
        for char_2 in buttons.iter() {
            let end = *mapping.get(&char_2).unwrap();
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0),origin,vec![]));
            let mut all_paths = FxHashSet::default();
            let mut visited_distance = FxHashMap::default();
            let mut end_distance = u64::MAX;
            while let Some((distance, pos, history)) = queue.pop() {
                if distance.0 > end_distance {
                    break;
                }
                if distance.0 > *visited_distance.get(&pos).unwrap_or(&u64::MAX) {
                    continue;
                }
                if pos == end {
                    all_paths.insert(history);
                    end_distance = end_distance.min(distance.0);
                    continue;
                }
                visited_distance.insert(pos, distance.0);
                for dir in directions {
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if positions.contains(&next_pos) {
                        if distance.0 + 1 <= *visited_distance.get(&next_pos).unwrap_or(&u64::MAX) {
                            let mut new_history = history.clone();
                            new_history.push(dir);
                            let mut extra_distance = 1;
                            if *history.last().unwrap_or(&dir) != dir {
                                extra_distance += 100;
                            }
                            queue.push((Reverse(distance.0+extra_distance), next_pos, new_history));
                        }
                    }
                }
            }
            let mut shortest_len = usize::MAX;
            for p in all_paths.iter() {
                shortest_len = shortest_len.min(p.len());
            }
            for path in all_paths {
                if path.len() > shortest_len {
                    continue;
                }
                let mut path_chars = vec![];
                for step in path {
                    let char_step = match step {
                        (0,1) => '>',
                        (1,0) => 'v',
                        (-1,0) => '^',
                        _ => '<',
                    };
                    path_chars.push(char_step);

                }
                let path_string: String = path_chars.into_iter().collect();
                char_paths.entry((*char_1,*char_2)).or_insert(vec![]).push(path_string);
            }            
        }
    }
    char_paths.insert(('v','A'), vec!["^>".to_string()]);
    char_paths.insert(('A','v'), vec!["<v".to_string()]);
    char_paths.insert(('>','^'), vec!["<^".to_string()]);
    char_paths.insert(('^','>'), vec!["v>".to_string()]);

    char_paths
}

fn split_into_blocks(min_string: &String) -> Vec<String> {
    let mut blocks = min_string.split('A').map(|x| {let mut y = x.to_string(); y.push_str("A"); y}).collect::<Vec<String>>();
    blocks.pop();
    blocks
}


#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    let instructions = parse(input);
    let mapping_1 = all_paths_1();
    let mapping_2 = all_paths_2();
    
    let mut blocks = vec![];
    for k in mapping_1.values() {
        for v in k {
            let mut vb = v.clone();
            vb.push_str("A");
            blocks.push(vb);
        }
    }

    let mut block_dictionary = FxHashMap::default();
    for b in blocks.iter() {
        let mut next_robot_instr = vec![];
        if b.len() == 1 {
            next_robot_instr.push('A');
            block_dictionary.insert(b.clone(), String::from_iter(next_robot_instr));
            continue;
        }
        let mut todo_vec = vec!['A'];
        todo_vec.extend(b.chars().collect::<Vec<char>>());
        for c in todo_vec.windows(2) {
            let char_instr = mapping_2.get(&(c[0],c[1])).unwrap().first().unwrap();
            next_robot_instr.extend(char_instr.chars().collect::<Vec<char>>());
            next_robot_instr.push('A');
        }
        block_dictionary.insert(b.clone(), String::from_iter(next_robot_instr));
    }
    println!("{:?}", block_dictionary);
    println!("{:?}", block_dictionary.keys());
    println!("{:?}", block_dictionary.len());
    let mut steps_ahead_8_full = FxHashMap::default();
    for key in blocks.iter() {
        let mut blocks_so_far = split_into_blocks(block_dictionary.get(key).unwrap());
        for depth in 0..7 {
            // println!("{:?}", blocks_so_far);
            let mut next_string = "".to_string();
            for b in blocks_so_far.iter() {
                next_string.push_str(block_dictionary.get(b as &str).unwrap());
            }
            blocks_so_far = split_into_blocks(&next_string);

        }
        let block_string = blocks_so_far.join("");
        steps_ahead_8_full.insert(key.clone(), block_string);
    }
    println!("{:?}", steps_ahead_8_full);
    for v in steps_ahead_8_full.values() {
        println!("{:?}", v.len());
    }
    let mut steps_ahead_16_ints = FxHashMap::default();
    for key in blocks.iter() {
        let mut length = 0;
        let blocks_so_far = split_into_blocks(steps_ahead_8_full.get(key).unwrap());
        for p in blocks_so_far {
            length += steps_ahead_8_full.get(&p as &str).unwrap().len();
        }
        steps_ahead_16_ints.insert(key, length);
    }
    println!("{:?}", steps_ahead_16_ints);
    // println!("{:?}", mapping_2);

    let mut outcome = 0;
    // println!("{:?}", mapping_2.keys());
    for code in instructions {
        let mut remaining = vec!['A'];
        let code_vec: Vec<char> = code.chars().collect();
        remaining.extend(code_vec);
        let remaining_vec_deque = VecDeque::from_iter(remaining.windows(2).map(|x| (x[0],x[1])));
        let min_vectors1 = recurse_1(remaining_vec_deque, &mapping_1);
        
        
        
        let min_vectors_hashset = FxHashSet::from_iter(min_vectors1);
        // println!("{:?}, {:?}", code, min_vectors_hashset);
        let mut vectors = vec![];
        for v in  min_vectors_hashset {
            let s = String::from_iter(v);
            let t = split_into_blocks(&s);
            let mut output_string = "".to_string();
            for b in t {
                output_string.push_str(block_dictionary.get(&b as &str).unwrap());
            }
            vectors.push(output_string);
        }

        let mut next_vectors = vec![];
        for v in  vectors {
            let t = split_into_blocks(&v);
            let mut output_string = "".to_string();
            for b in t {
                output_string.push_str(steps_ahead_8_full.get(&b as &str).unwrap());
            }
            next_vectors.push(output_string);
        }
        let mut next_next_vectors = vec![];

        for v in next_vectors {
            let t = split_into_blocks(&v);
            let mut output_string = 0;
            for b in t {
                output_string += steps_ahead_16_ints.get(&b).unwrap();
            }
            next_next_vectors.push(output_string);
        }

        let code_digit: u64 = code[0..3].parse().unwrap();
        let min_length = *next_next_vectors.iter().min().unwrap();
        // let min_length = next_vectors.iter().min_by_key(|x| x.len()).unwrap().len();
        println!("{:?} {:?}", code, min_length as u64);

        outcome += min_length as u64 * code_digit;

    }
    outcome
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
        assert_eq!(part1(&contents), 126384 );
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 0);
    }
}