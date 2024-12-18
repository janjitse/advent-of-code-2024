use std::time::SystemTime;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(": ")
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
                .next()
                .unwrap()
        })
        .collect::<Vec<u64>>();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .next()
        .unwrap();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (output1, output2)
}

#[derive(Debug)]
enum Opcode {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc(u64),
    Out(u64),
    Bdv(u64),
    Cdv(u64),
}

impl std::convert::From<(u64, u64)> for Opcode {
    fn from((n, m): (u64, u64)) -> Self {
        match n {
            0 => Opcode::Adv(m),
            1 => Opcode::Bxl(m),
            2 => Opcode::Bst(m),
            3 => Opcode::Jnz(m),
            4 => Opcode::Bxc(m),
            5 => Opcode::Out(m),
            6 => Opcode::Bdv(m),
            7 => Opcode::Cdv(m),
            _ => panic!(),
        }
    }
}

fn combo(v: u64, registers: &[u64]) -> u64 {
    if v < 4 {
        return v;
    } else if v < 7 {
        return registers[v as usize - 4];
    }
    panic!();
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let (registers, instructions) = parse(input);
    let opcodes: Vec<Opcode> = instructions
        .chunks(2)
        .map(|x| Opcode::from((x[0], x[1])))
        .collect();
    let output = run(registers[0], &opcodes);
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u64 {
    let (_, instructions) = parse(input);
    let orig_instructions = instructions.clone();
    let opcodes: Vec<Opcode> = instructions
        .chunks(2)
        .map(|x| Opcode::from((x[0], x[1])))
        .collect();
    let output = recurse(orig_instructions.len() - 1, &orig_instructions, 1, &opcodes);
    output.unwrap()
}

#[aoc(day17, part2, all)]
fn part2_all(input: &str) -> u64 {
    let (_, mut instructions) = parse(input);
    let bxl1_loc = 3;
    let bxl2_loc = 9;
    let bxc_loc = 7;

    let mut solvable: u64 = 0;
    let mut unsolvable: u64 = 0;

    for bxl1_val in 0..8 {
        for bxl2_val in 0..8 {
            for bxc_val in 0..8 {
                instructions[bxl1_loc] = bxl1_val;
                instructions[bxl2_loc] = bxl2_val;
                instructions[bxc_loc] = bxc_val;
                let orig_instructions = instructions.clone();
                let opcodes: Vec<Opcode> = instructions
                    .chunks(2)
                    .map(|x| Opcode::from((x[0], x[1])))
                    .collect();
                match recurse(orig_instructions.len() - 1, &orig_instructions, 1, &opcodes) {
                    None => {
                        unsolvable += 1;
                    }
                    Some(_) => {
                        println!("Solvable: {:?}", opcodes);
                        solvable += 1;
                    }
                }
            }
        }
    }
    println!("Solvable: {:?}, unsolvable: {:?}", solvable, unsolvable);
    0
}

fn recurse(depth: usize, target: &Vec<u64>, so_far: u64, opcodes: &[Opcode]) -> Option<u64> {
    if depth == 0 {
        if run(so_far, opcodes) == *target {
            return Some(so_far);
        } else {
            return None;
        }
    }
    let mut min_outcome = u64::MAX;
    for trial in 8 * so_far..8 * so_far + 128 {
        if run(trial, opcodes) == target[depth - 1..] {
            if let Some(outcome) = recurse(depth - 1, target, trial, opcodes) {
                min_outcome = min_outcome.min(outcome);
            }
        }
    }
    if min_outcome < u64::MAX {
        return Some(min_outcome);
    }
    None
}

fn run(register_a: u64, opcodes: &[Opcode]) -> Vec<u64> {
    let mut registers = vec![register_a, 0, 0];
    let mut cur_pointer = 0;
    let mut output = vec![];
    loop {
        match opcodes[cur_pointer] {
            Opcode::Adv(v) => {
                registers[0] >>= combo(v, &registers);
            }
            Opcode::Bxl(v) => {
                registers[1] ^= v;
            }
            Opcode::Bst(v) => {
                registers[1] = combo(v, &registers) % 8;
            }
            Opcode::Jnz(v) => {
                if registers[0] != 0 {
                    cur_pointer = v as usize;
                    continue;
                }
            }
            Opcode::Bxc(_v) => {
                registers[1] ^= registers[2];
            }
            Opcode::Out(v) => {
                output.push(combo(v, &registers) % 8);
            }
            Opcode::Bdv(v) => {
                registers[1] = registers[0] >> combo(v, &registers);
            }
            Opcode::Cdv(v) => {
                registers[2] = registers[0] >> combo(v, &registers);
            }
        }
        cur_pointer += 1;
        if cur_pointer >= opcodes.len() {
            break;
        }
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
        assert_eq!(part1(&contents), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_1_tiny() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), "4,2,5,6,7,7,7,7,3,1,0".to_string());
    }
    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 117440);
    }
}
