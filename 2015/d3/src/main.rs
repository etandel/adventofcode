use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

type Dim = i64;
type Point = (Dim, Dim);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(c: char) -> Dir {
        match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Invalid char {}", c),
        }
    }
}

impl Dir {
    fn delta(&self) -> Point {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (1, 0),
            Self::Right => (-1, 0),
        }
    }
}

fn parse_path<P>(path: P) -> Vec<Dir>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(str::chars)
        .map(Dir::from)
        .collect()
}

fn part1() {
    let path = parse_path("input.txt");

    let mut pos = Point::default();

    let mut visited: HashSet<Point> = HashSet::with_capacity(path.len());

    visited.insert(pos);
    for (dx, dy) in path.iter().map(Dir::delta) {
        pos = (pos.0 + dx, pos.1 + dy);
        visited.insert(pos);
    }

    println!("{}", visited.len());
}

fn part2() {
    let path = parse_path("input.txt");

    let mut visited: HashSet<Point> = HashSet::with_capacity(path.len());

    let mut santa_pos = Point::default();
    let mut robo_pos = Point::default();

    let mut is_robo_santa = false;

    visited.insert(santa_pos);
    for (dx, dy) in path.iter().map(Dir::delta) {
        let pos = if is_robo_santa {
            robo_pos = (robo_pos.0 + dx, robo_pos.1 + dy);
            robo_pos
        } else {
            santa_pos = (santa_pos.0 + dx, santa_pos.1 + dy);
            santa_pos
        };
        visited.insert(pos);
        is_robo_santa = !is_robo_santa;
    }

    println!("{}", visited.len());
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
mod tests {}
