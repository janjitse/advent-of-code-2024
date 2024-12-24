use std::time::SystemTime;

fn parse(input: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(": ").map(|x| x.to_string()).collect())
        .collect();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (output1, output2)
}

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Gate {
    left: String,
    right: String,
    op: BitOp,
    output: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BitOp {
    AND,
    XOR,
    OR,
}

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    let (start_values, rules) = parse(input);
    let mut values = FxHashMap::default();
    for s in start_values {
        let key = s[0].clone();
        let value: u32 = s[1].parse().unwrap();
        values.insert(key, value);
    }
    let mut gates = vec![];
    for r in rules {
        let p = r.split_ascii_whitespace().collect::<Vec<&str>>();
        let op = match p[1] {
            "AND" => BitOp::AND,
            "XOR" => BitOp::XOR,
            "OR" => BitOp::OR,
            _ => panic!(),
        };
        gates.push(Gate {
            left: p[0].to_string(),
            right: p[2].to_string(),
            op,
            output: p[4].to_string(),
        });
    }
    loop {
        let mut anything_done = false;
        for g in gates.iter().rev() {
            if values.contains_key(&g.left)
                && values.contains_key(&g.right)
                && !values.contains_key(&g.output)
            {
                anything_done = true;
                let left_v = values.get(&g.left).unwrap();
                let right_v = values.get(&g.right).unwrap();
                let output_v = match g.op {
                    BitOp::AND => left_v & right_v,
                    BitOp::XOR => left_v ^ right_v,
                    BitOp::OR => left_v | right_v,
                };
                values.insert(g.output.clone(), output_v);
            }
        }
        if !anything_done {
            break;
        }
    }

    let mut end_values = vec![];

    for (key, value) in values {
        if key.starts_with("z") {
            end_values.push((key, value));
        }
    }
    end_values.sort_unstable();
    println!("{:?}", end_values);
    let mut output = 0;
    for (_, value) in end_values.into_iter().rev() {
        output = output * 2 + value as u64;
    }

    output
}

fn simulate_adder(
    input_x: Vec<u32>,
    input_y: Vec<u32>,
    gates: &[Gate],
) -> (Vec<(String, u32)>, FxHashMap<String, u32>) {
    let mut values = FxHashMap::default();
    for (idx, val) in input_x.into_iter().enumerate() {
        let key = format!("x{:0>2}", idx);
        values.insert(key, val);
    }
    for (idx, val) in input_y.into_iter().enumerate() {
        let key = format!("y{:0>2}", idx);
        values.insert(key, val);
    }
    loop {
        let mut anything_done = false;
        for g in gates.iter() {
            if values.contains_key(&g.left)
                && values.contains_key(&g.right)
                && !values.contains_key(&g.output)
            {
                anything_done = true;
                let left_v = values.get(&g.left).unwrap();
                let right_v = values.get(&g.right).unwrap();
                let output_v = match g.op {
                    BitOp::AND => left_v & right_v,
                    BitOp::XOR => left_v ^ right_v,
                    BitOp::OR => left_v | right_v,
                };
                values.insert(g.output.clone(), output_v);
            }
        }
        if !anything_done {
            break;
        }
    }
    let mut end_values = vec![];

    for (key, value) in values.iter() {
        if key.starts_with("z") {
            end_values.push((key.clone(), *value));
        }
    }
    end_values.sort_unstable();
    (end_values, values)
}

fn vec_to_nr<T>(v: &[(T, u32)]) -> u64 {
    let mut output = 0;
    for (_, value) in v.iter().rev() {
        output = output * 2 + *value as u64;
    }
    output
}

fn nr_to_vec(mut nr: u64, len: usize) -> Vec<u32> {
    let mut output = Vec::with_capacity(len);
    for _ in 0..len {
        output.push((nr % 2) as u32);
        nr /= 2;
    }
    output
}

#[aoc(day24, part2)]
fn part2(input: &str) -> String {
    let (start_values, rules) = parse(input);
    let mut values = FxHashMap::default();
    for s in start_values {
        let key = s[0].clone();
        let value: u32 = s[1].parse().unwrap();
        values.insert(key, value);
    }
    let mut gates = vec![];
    for r in rules {
        let p = r.split_ascii_whitespace().collect::<Vec<&str>>();
        let op = match p[1] {
            "AND" => BitOp::AND,
            "XOR" => BitOp::XOR,
            "OR" => BitOp::OR,
            _ => panic!(),
        };
        gates.push(Gate {
            left: p[0].to_string(),
            right: p[2].to_string(),
            op,
            output: p[4].to_string(),
        });
    }
    let mut input_x = 0;
    let mut input_y = 0;
    let mut prev_keys = FxHashSet::default();

    let mut output_vec = vec![];

    for idx in 0..45 {
        let mut correct = true;
        input_x += 1 << idx;
        let (output, all_values1) =
            simulate_adder(nr_to_vec(input_x, 45), nr_to_vec(input_y, 45), &gates);
        let mut relevant_keys = vec![];

        let relevant_values = all_values1
            .into_iter()
            .filter(|x| x.1 == 1)
            .collect::<Vec<(String, u32)>>();

        relevant_keys.extend(
            relevant_values
                .into_iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
        );
        relevant_keys.push(format!("z{:0>2}", idx));

        if input_x + input_y != vec_to_nr(&output) {
            correct = false;
        }
        input_y += 1 << idx;

        let (output, all_values2) =
            simulate_adder(nr_to_vec(input_x, 45), nr_to_vec(input_y, 45), &gates);

        let relevant_values = all_values2
            .into_iter()
            .filter(|x| x.1 == 1)
            .collect::<Vec<(String, u32)>>();
        relevant_keys.extend(
            relevant_values
                .into_iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
        );
        relevant_keys.push(format!("z{:0>2}", idx + 1));

        if input_x + input_y != vec_to_nr(&output) {
            correct = false;
        }
        if !correct {
            let mut relevant_keys_set = FxHashSet::from_iter(relevant_keys.iter().cloned());
            relevant_keys_set = relevant_keys_set.difference(&prev_keys).cloned().collect();
            let mut relevant_gates_idx = vec![];
            for (idx, g) in gates.iter().enumerate() {
                if relevant_keys_set.contains(&g.output) {
                    relevant_gates_idx.push(idx);
                }
            }
            'outer: for swap1 in relevant_gates_idx.iter() {
                for swap2 in relevant_gates_idx.iter() {
                    if (gates[*swap1].output.starts_with("z") && gates[*swap2].op != BitOp::XOR)
                        || (gates[*swap2].output.starts_with("z") && gates[*swap1].op != BitOp::XOR)
                    {
                        continue;
                    }
                    if swap1 != swap2
                        && gates[*swap1].left != gates[*swap2].output
                        && gates[*swap1].right != gates[*swap2].output
                        && gates[*swap2].left != gates[*swap1].output
                        && gates[*swap2].right != gates[*swap1].output
                    {
                        (gates[*swap1].output, gates[*swap2].output) =
                            (gates[*swap2].output.clone(), gates[*swap1].output.clone());
                        let (output2, _) = simulate_adder(
                            nr_to_vec(input_x, 45),
                            nr_to_vec(input_y - (1 << idx), 45),
                            &gates,
                        );
                        let (output3, _) = simulate_adder(
                            nr_to_vec(input_x + (1 << idx), 45),
                            nr_to_vec(input_y, 45),
                            &gates,
                        );
                        if vec_to_nr(&output2) == input_x + input_y - (1 << idx)
                            && vec_to_nr(&output3) == input_x + input_y + (1 << idx)
                        {
                            println!(
                                "Solution found: {:?}, {:?}",
                                gates[*swap1].output, gates[*swap2].output
                            );
                            output_vec.push(gates[*swap1].output.clone());
                            output_vec.push(gates[*swap2].output.clone());
                            break 'outer;
                        }
                        (gates[*swap1].output, gates[*swap2].output) =
                            (gates[*swap2].output.clone(), gates[*swap1].output.clone());
                    }
                }
            }
        }
        for k in relevant_keys {
            prev_keys.insert(k);
        }
    }
    output_vec.sort_unstable();

    output_vec.join(",").to_string()
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
        assert_eq!(part1(&contents), 4);
    }

    #[test]
    fn test_1b() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 2024);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_medium.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(
            part2(&contents),
            "aaa,aoc,bbb,ccc,eee,ooo,z24,z99".to_string()
        );
    }
}
