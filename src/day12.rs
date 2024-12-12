use std::{collections::VecDeque, time::SystemTime};

use fxhash::FxHashSet;

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

fn generate_regions(mut grid: Vec<Vec<char>>) -> Vec<FxHashSet<(usize, usize)>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut regions = vec![];
    for row_idx in 0..height {
        for col_idx in 0..width {
            if grid[row_idx][col_idx] != '.' {
                let current_char = grid[row_idx][col_idx];
                let current_region = flood_fill(&mut grid, current_char, (row_idx, col_idx));
                regions.push(current_region);
            }
        }
    }
    regions
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let regions = generate_regions(x);

    let mut output = 0;
    for region in regions {
        let area = region.len() as u64;
        let mut perimeter = 0;
        for c in region.iter() {
            for dir in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
                if !region.contains(&(c.0.wrapping_add(dir.0), c.1.wrapping_add(dir.1))) {
                    perimeter += 1;
                }
            }
        }
        output += area * perimeter;
    }

    output
}

#[aoc(day12, part1, conv)]
fn part1_conv(input: &str) -> u64 {
    let x = parse(input);
    let height = x.len();
    let width = x[0].len();
    let mut edges: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if row_idx == 0 || x[row_idx - 1][col_idx] != val {
                edges[row_idx][col_idx] += 1;
            }
            if row_idx == height - 1 || x[row_idx + 1][col_idx] != val {
                edges[row_idx][col_idx] += 1;
            }
            if col_idx == 0 || x[row_idx][col_idx - 1] != val {
                edges[row_idx][col_idx] += 1;
            }
            if col_idx == width - 1 || x[row_idx][col_idx + 1] != val {
                edges[row_idx][col_idx] += 1;
            }
        }
    }

    let regions = generate_regions_vec(x);

    let mut output = 0;
    for region in regions {
        let area = region.len() as u64;
        let perimeter = region
            .into_iter()
            .map(|c| edges[c.0][c.1] as u64)
            .sum::<u64>();
        output += area * perimeter;
    }

    output
}

#[aoc(day12, part2, conv)]
fn part2_conv(input: &str) -> u64 {
    let x = parse(input);
    let height = x.len();
    let width = x[0].len();
    let mut corners: Vec<Vec<u8>> = vec![vec![0; width]; height];
    let kernels_inner = [
        ((usize::MAX, 0), (0, 1), (usize::MAX, 1)),
        ((0, 1), (1, 0), (1, 1)),
        ((1, 0), (0, usize::MAX), (1, usize::MAX)),
        ((0, usize::MAX), (usize::MAX, 0), (usize::MAX, usize::MAX)),
    ];

    for (row_idx, row) in x.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            for k in kernels_inner.iter() {
                let corner = (row_idx.wrapping_add(k.2 .0), col_idx.wrapping_add(k.2 .1));
                let b1 = (row_idx.wrapping_add(k.0 .0), col_idx.wrapping_add(k.0 .1));
                let b2 = (row_idx.wrapping_add(k.1 .0), col_idx.wrapping_add(k.1 .1));
                let b1_val = match b1.0 >= height || b1.1 >= width {
                    true => '.',
                    false => x[b1.0][b1.1],
                };
                let b2_val = match b2.0 >= height || b2.1 >= width {
                    true => '.',
                    false => x[b2.0][b2.1],
                };
                let corner_val = match corner.0 >= height || corner.1 >= width {
                    true => '.',
                    false => x[corner.0][corner.1],
                };
                if &corner_val != val && &b1_val == val && &b2_val == val {
                    corners[row_idx][col_idx] += 1;
                }
                if &b1_val != val && &b2_val != val {
                    corners[row_idx][col_idx] += 1;
                }
            }
        }
    }

    let regions = generate_regions_vec(x);

    let mut output = 0;
    for region in regions {
        let area = region.len() as u64;
        let perimeter = region
            .into_iter()
            .map(|c| corners[c.0][c.1] as u64)
            .sum::<u64>();
        output += area * perimeter;
    }

    output
}

fn flood_fill(
    x: &mut [Vec<char>],
    cur_char: char,
    start_loc: (usize, usize),
) -> FxHashSet<(usize, usize)> {
    let mut region = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(start_loc);
    region.insert(start_loc);
    while let Some(next_c) = queue.pop_front() {
        for dir in [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)] {
            let next_loc = (next_c.0.wrapping_add(dir.0), next_c.1.wrapping_add(dir.1));
            if next_loc.0 < x.len()
                && next_loc.1 < x[0].len()
                && x[next_loc.0][next_loc.1] == cur_char
            {
                queue.push_back(next_loc);
                region.insert(next_loc);
                x[next_loc.0][next_loc.1] = '.';
            }
        }
    }
    region
}

fn flood_fill_vec(
    x: &mut [Vec<char>],
    cur_char: char,
    start_loc: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut region = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(start_loc);
    region.push(start_loc);
    x[start_loc.0][start_loc.1] = '.';
    while let Some(next_c) = queue.pop_front() {
        for dir in [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)] {
            let next_loc = (next_c.0.wrapping_add(dir.0), next_c.1.wrapping_add(dir.1));
            if next_loc.0 < x.len()
                && next_loc.1 < x[0].len()
                && x[next_loc.0][next_loc.1] == cur_char
            {
                queue.push_back(next_loc);
                region.push(next_loc);
                x[next_loc.0][next_loc.1] = '.';
            }
        }
    }
    region
}

fn generate_regions_vec(mut grid: Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut regions = vec![];
    for row_idx in 0..height {
        for col_idx in 0..width {
            if grid[row_idx][col_idx] != '.' {
                let current_char = grid[row_idx][col_idx];
                let current_region = flood_fill_vec(&mut grid, current_char, (row_idx, col_idx));
                regions.push(current_region);
            }
        }
    }
    regions
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let x = parse(input);
    let regions = generate_regions(x);

    let mut output = 0;
    for region in regions {
        let area = region.len() as u64;
        let mut corners = 0;
        for c in region.iter() {
            let mut conv = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
            for dir in [
                (0, 1),
                (1, 1),
                (1, 0),
                (usize::MAX, 0),
                (0, usize::MAX),
                (1, usize::MAX),
                (usize::MAX, usize::MAX),
                (usize::MAX, 1),
            ] {
                if !region.contains(&(c.0.wrapping_add(dir.0), c.1.wrapping_add(dir.1))) {
                    conv[1usize.wrapping_add(dir.0)][1usize.wrapping_add(dir.1)] = 1;
                }
            }
            // outer corners
            for w in [(0, 1), (1, 2), (2, 1), (1, 0), (0, 1)].windows(2) {
                if conv[w[0].0][w[0].1] == 1 && conv[w[1].0][w[1].1] == 1 {
                    corners += 1;
                }
            }

            //inner corners
            for w in [
                ((0, 1), (1, 2), (0, 2)),
                ((1, 2), (2, 1), (2, 2)),
                ((2, 1), (1, 0), (2, 0)),
                ((1, 0), (0, 1), (0, 0)),
            ] {
                if conv[w.0 .0][w.0 .1] == 0
                    && conv[w.1 .0][w.1 .1] == 0
                    && conv[w.2 .0][w.2 .1] == 1
                {
                    corners += 1;
                }
            }
        }
        output += area * corners;
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
    fn test_1_conv() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1_conv(&contents), 140);
    }

    #[test]
    fn test_1_conv_vec() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part1_conv(&contents), 140);
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
