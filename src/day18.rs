use std::time::SystemTime;

fn parse(input: &str) -> Vec<Vec<u64>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(",").map(|x| x.parse::<u64>().unwrap()).collect())
        .collect::<Vec<Vec<u64>>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

use fxhash::FxHashSet;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Item {
    dist: u64,
    loc: (usize, usize),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.dist).cmp(&Reverse(other.dist))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Reverse(self.dist).cmp(&Reverse(other.dist)))
    }
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    let x = parse(input);
    let nr_bytes = match x.len() {
        25 => 12,
        3450 => 1024,
        250999 => 1024,
        251001 => 1024,
        6254999 => 1024,
        6255000 => 1024,
        _ => panic!(),
    };
    let visited = FxHashSet::from_iter(
        x.iter()
            .take(nr_bytes)
            .map(|item| (item[0] as usize, item[1] as usize)),
    );

    let end = match x.len() {
        25 => (6, 6),
        3450 => (70, 70),
        250999 => (500, 500),
        251001 => (500, 500),
        6254999 => (2500, 2500),
        6255000 => (2500, 2500),
        _ => panic!(),
    };
    dijkstra((0, 0), end, visited).unwrap()
}

fn dijkstra(
    start: (usize, usize),
    end: (usize, usize),
    mut visited: FxHashSet<(usize, usize)>,
) -> Option<u64> {
    let mut p_q = BinaryHeap::new();
    let dir = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    p_q.push(Item {
        dist: 0,
        loc: start,
    });
    while let Some(Item {
        dist,
        loc: next_pos,
    }) = p_q.pop()
    {
        if visited.contains(&next_pos) {
            continue;
        }
        if next_pos == end {
            return Some(dist);
        }
        visited.insert(next_pos);
        for d in &dir {
            let trial = (next_pos.0.wrapping_add(d.0), next_pos.1.wrapping_add(d.1));
            if trial.0 <= end.0 && trial.1 <= end.1 && !visited.contains(&trial) {
                p_q.push(Item {
                    dist: dist + 1,
                    loc: trial,
                });
            }
        }
    }
    None
}

#[aoc(day18, part2)]
fn part2(input: &str) -> String {
    let x = parse(input);
    let end = match x.len() {
        25 => (6, 6),
        3450 => (70, 70),
        250999 => (500, 500),
        251001 => (500, 500),
        6254999 => (2500, 2500),
        6255000 => (2500, 2500),
        6255001 => (2500, 2500),
        _ => panic!(),
    };
    let mut low = 0;
    let mut high = x.len() - 1;
    while high > low {
        let mid = (high + low + 1) / 2;
        let visited = FxHashSet::from_iter(
            x.iter()
                .take(mid)
                .map(|item| (item[0] as usize, item[1] as usize)),
        );

        if dijkstra((0, 0), end, visited).is_some() {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    format!("{},{}", x[high][0], x[high][1])
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
        assert_eq!(part1(&contents), 22);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2024/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part2(&contents), "6,1".to_string());
    }
}
