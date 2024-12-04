fn parse(input: &str) -> Vec<Vec<char>> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.chars().collect())
    }
    return output;
}

#[aoc(day4, part1)]
fn part1(input: &str) -> i32 {
    let vec_c = parse(input);
    let mut total = 0;
    for dir in [
        (0, 1),
        (1, 0),
        (1, 1),
        (usize::MAX, 0),
        (usize::MAX, 1),
        (usize::MAX, usize::MAX),
        (1, usize::MAX),
        (0, usize::MAX),
    ] {
        for start_x in 0..vec_c.len() {
            for start_y in 0..vec_c[0].len() {
                let mut test: Vec<char> = vec![];
                let mut pos_x = start_x;
                let mut pos_y = start_y;
                for let_idx in 0..4 {
                    test.push(vec_c[pos_x][pos_y]);
                    pos_x = pos_x.wrapping_add(dir.0);
                    pos_y = pos_y.wrapping_add(dir.1);
                    if pos_x >= vec_c.len() || pos_y >= vec_c[0].len() {
                        break;
                    }
                }
                if test == vec!['X', 'M', 'A', 'S'] {
                    total += 1;
                    // println!("{:?}, {:?},{:?}", dir, start_x, start_y);
                }
            }
        }
    }
    return total;
}

#[aoc(day4, part2)]
fn part2(input: &str) -> i32 {
    let vec_c = parse(input);
    let mut total = 0;
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
                    test.push(vec_c[pos_x][pos_y]);
                }
                if test == vec!['S', 'M', 'M', 'S']
                    || test == vec!['S', 'S', 'M', 'M']
                    || test == vec!['M', 'S', 'S', 'M']
                    || test == vec!['M', 'M', 'S', 'S']
                {
                    total += 1;
                }
            }
        }
    }
    return total;
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
