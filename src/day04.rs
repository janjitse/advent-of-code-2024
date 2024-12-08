fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let vec_c = parse(input);
    let mut total = 0;
    let mas_vec = vec!['M', 'A', 'S'];
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (usize::MAX, 0),
        (usize::MAX, 1),
        (usize::MAX, usize::MAX),
        (1, usize::MAX),
        (0, usize::MAX),
    ];
    for start_x in 0..vec_c.len() {
        for start_y in 0..vec_c[0].len() {
            if vec_c[start_x][start_y] == 'X' {
                for dir in directions {
                    let mut test: Vec<char> = vec![];
                    let mut pos_x = start_x;
                    let mut pos_y = start_y;
                    for &mas_char in mas_vec.iter() {
                        pos_x = pos_x.wrapping_add(dir.0);
                        pos_y = pos_y.wrapping_add(dir.1);
                        if pos_x >= vec_c.len() || pos_y >= vec_c[0].len() {
                            break;
                        }
                        let test_c = vec_c[pos_x][pos_y];
                        if test_c == mas_char {
                            test.push(test_c);
                        } else {
                            break;
                        }
                    }
                    if test == mas_vec {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let vec_c = parse(input);
    let mut total = 0;
    let mmss = vec!['M', 'M', 'S', 'S'];
    let smms = vec!['S', 'M', 'M', 'S'];
    let ssmm = vec!['S', 'S', 'M', 'M'];
    let mssm = vec!['M', 'S', 'S', 'M'];
    for start_x in 1..vec_c.len() - 1 {
        for start_y in 1..vec_c[0].len() - 1 {
            let mut test: Vec<char> = vec![];
            if vec_c[start_x][start_y] == 'A' {
                for around in [
                    (1, 1),
                    (1, usize::MAX),
                    (usize::MAX, usize::MAX),
                    (usize::MAX, 1),
                ] {
                    let pos_x = start_x.wrapping_add(around.0);
                    let pos_y = start_y.wrapping_add(around.1);
                    let test_c = vec_c[pos_x][pos_y];
                    if test_c == 'S' || test_c == 'M' {
                        test.push(test_c);
                    } else {
                        break;
                    }
                }
                if test == mmss || test == ssmm || test == mssm || test == smms {
                    total += 1;
                }
            }
        }
    }
    total
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
        assert_eq!(part1(&contents), 18);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), 9);
    }
}
