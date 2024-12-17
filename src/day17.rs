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
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .map(|y| y.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .next()
        .unwrap();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    return (output1, output2);
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

fn combo(v: u64, registers: &Vec<u64>) -> u64 {
    if v < 4 {
        return v;
    }
    if v < 7 {
        return registers[v as usize - 4];
    }
    panic!();
}

#[aoc(day17, part1)]
fn part1(input: &str) -> u64 {
    let (mut registers, instructions) = parse(input);
    let opcodes: Vec<Opcode> = instructions
        .chunks(2)
        .map(|x| match x[0] {
            0 => Opcode::Adv(x[1]),
            1 => Opcode::Bxl(x[1]),
            2 => Opcode::Bst(x[1]),
            3 => Opcode::Jnz(x[1]),
            4 => Opcode::Bxc(x[1]),
            5 => Opcode::Out(x[1]),
            6 => Opcode::Bdv(x[1]),
            7 => Opcode::Cdv(x[1]),
            _ => panic!(),
        })
        .collect();
    println!("{:?}", opcodes);
    // return 0;
    let mut cur_pointer = 0;
    loop {
        match opcodes[cur_pointer] {
            Opcode::Adv(v) => {
                registers[0] >>= combo(v, &registers);
            }
            Opcode::Bxl(v) => {
                registers[1] = registers[1] ^ v;
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
                registers[1] = registers[1] ^ registers[2];
            }
            Opcode::Out(v) => {
                print!("{:?},", combo(v, &registers) % 8);
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

    return 0;
}

#[aoc(day17, part2)]
fn part2(input: &str) -> u64 {
    let (mut registers, instructions) = parse(input);
    let orig_instructions = instructions.clone();
    let opcodes: Vec<Opcode> = instructions
        .chunks(2)
        .map(|x| match x[0] {
            0 => Opcode::Adv(x[1]),
            1 => Opcode::Bxl(x[1]),
            2 => Opcode::Bst(x[1]),
            3 => Opcode::Jnz(x[1]),
            4 => Opcode::Bxc(x[1]),
            5 => Opcode::Out(x[1]),
            6 => Opcode::Bdv(x[1]),
            7 => Opcode::Cdv(x[1]),
            _ => panic!(),
        })
        .collect();
    println!("{:?}", opcodes);
    // return 0;

    // let old_registers = registers.clone();
    let output = recurse(orig_instructions.len() - 1, &orig_instructions, 1, &opcodes);

    // println!("{:?}", output.len());
    return output.unwrap();
}

fn recurse(depth: usize, target: &Vec<u64>, so_far: u64, opcodes: &Vec<Opcode>) -> Option<u64> {
    if depth == 0 {
        if run(so_far, opcodes) == *target {
            return Some(so_far);
        } else {
            return None;
        }
    }
    let mut min_outcome = u64::MAX;
    for trial in 8 * so_far..8 * so_far + 8 {
        if run(trial, opcodes) == &target[depth - 1..] {
            if let Some(outcome) = recurse(depth - 1, target, trial, opcodes) {
                min_outcome = min_outcome.min(outcome);
            }
        }
    }
    if min_outcome < u64::MAX {
        return Some(min_outcome);
    }

    return None;
}

fn run(register_a: u64, opcodes: &Vec<Opcode>) -> Vec<u64> {
    let mut registers = vec![register_a, 0, 0];
    let mut cur_pointer = 0;
    let mut output = vec![];
    loop {
        // println!("{:?}", cur_pointer);
        match opcodes[cur_pointer] {
            Opcode::Adv(v) => {
                registers[0] >>= combo(v, &registers);
            }
            Opcode::Bxl(v) => {
                registers[1] = registers[1] ^ v;
            }
            Opcode::Bst(v) => {
                registers[1] = combo(v, &registers) % 8;
            }
            Opcode::Jnz(v) => {
                // println!("A: {:?}", registers[0]);
                if registers[0] != 0 {
                    // println!("v: {:?}", v);
                    cur_pointer = v as usize;
                    continue;
                }
            }
            Opcode::Bxc(_v) => {
                registers[1] = registers[1] ^ registers[2];
            }
            Opcode::Out(v) => {
                // println!("{:?}, {:?},", a, combo(v, &registers) % 8);
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
    // println!("{:?}, {:?}", a, output);
    return output;
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
    fn test_1_tiny() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_tiny.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1(&contents), 0);
    }
    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small2.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 0);
    }
}
