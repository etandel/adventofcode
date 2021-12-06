use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

type Id = usize;
type Len = usize;
type Pos = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    R,
    L,
    U,
    D,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'R' => Dir::R,
            'L' => Dir::L,
            'U' => Dir::U,
            'D' => Dir::D,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

type Wire = (Id, Vec<(Dir, Len)>);

fn parse_wire(id: Id, s: &str) -> Wire {
    let path = s
        .split(',')
        .map(|raw| {
            let distance = raw[1..].parse::<Len>().unwrap();
            (raw.chars().next().map(Dir::from).unwrap(), distance)
        })
        .collect();

    (id, path)
}

fn parse_wires<P>(path: P) -> Vec<Wire>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, raw)| parse_wire(i, raw))
        .collect()
}

fn part1() {
    let wires = parse_wires("input.txt");

    let mut counts: HashMap<Pos, HashSet<Id>> = HashMap::new();

    for (wire_id, path) in &wires {
        let mut pos = (0, 0);
        for (dir, len) in path {
            let (dx, dy): Pos = match dir {
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
                Dir::U => (0, 1),
                Dir::D => (0, -1),
            };

            for _ in 0..*len {
                pos = (pos.0 + dx, pos.1 + dy);
                counts
                    .entry(pos)
                    .or_insert_with(|| HashSet::with_capacity(2))
                    .insert(*wire_id);
            }
        }
    }

    let min_manhattan_distance_of_crossing = counts
        .iter()
        .filter_map(|(pos, c)| {
            if c.len() == wires.len() {
                Some(pos)
            } else {
                None
            }
        })
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();

    println!("{}", min_manhattan_distance_of_crossing);
}

fn part2() {
    let wires = parse_wires("input.txt");

    let mut counts: HashMap<Pos, HashSet<Id>> = HashMap::new();
    let mut visited_at: HashMap<Pos, HashMap<Id, usize>> = HashMap::new();

    for (wire_id, path) in &wires {
        let mut pos = (0, 0);
        let mut step = 1; // this is 1 because (0, 0) counts

        for (dir, len) in path {
            let (dx, dy): Pos = match dir {
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
                Dir::U => (0, 1),
                Dir::D => (0, -1),
            };

            for _ in 0..*len {
                pos = (pos.0 + dx, pos.1 + dy);
                let inserted = counts
                    .entry(pos)
                    .or_insert_with(|| HashSet::with_capacity(2))
                    .insert(*wire_id);

                if inserted {
                    visited_at
                        .entry(pos)
                        .or_insert_with(|| HashMap::with_capacity(2))
                        .insert(*wire_id, step);
                }

                step += 1;
            }
        }
    }

    let min_signal_delay: usize = counts
        .iter()
        .filter_map(|(pos, c)| {
            if c.len() == wires.len() {
                Some(pos)
            } else {
                None
            }
        })
        .map(|pos| visited_at.get(pos).unwrap().values().sum())
        .min()
        .unwrap();

    println!("{}", min_signal_delay);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_() {}
}
