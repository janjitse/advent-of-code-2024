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
                let current_region = flood_fill(&grid, current_char, (row_idx, col_idx));
                for d in current_region.clone().iter() {
                    grid[d.0][d.1] = '.';
                }
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

fn flood_fill(
    x: &[Vec<char>],
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
                && !region.contains(&next_loc)
            {
                queue.push_back(next_loc);
                region.insert(next_loc);
            }
        }
    }
    region
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
