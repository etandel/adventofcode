use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Dim = i64;

type Point = [Dim; 2];

fn parse_point(s: &str) -> Point {
    let parts = s
        .split(',')
        .map(Dim::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    [parts[0], parts[1]]
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start[0] == self.end[0]
    }

    fn is_vertical(&self) -> bool {
        self.start[1] == self.end[1]
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<_> = s.split(" -> ").collect();

        let start = parse_point(parts[0]);
        let end = parse_point(parts[1]);

        Ok(Self { start, end })
    }
}

impl<'a> IntoIterator for &'a Line {
    type Item = Point;
    type IntoIter = LineIter<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        LineIter::new(&self.start, &self.end)
    }
}

#[derive(Debug)]
struct LineIter<'a> {
    start: &'a Point,
    end: &'a Point,
    max_i: Dim,
    i: Dim,
    delta: Point,
}

fn get_delta(start: Dim, end: Dim) -> Dim {
    match end - start {
        0 => 0,
        x if x > 0 => 1,
        _ => -1,
    }
}

impl<'a> LineIter<'a> {
    fn new(start: &'a Point, end: &'a Point) -> Self {
        LineIter {
            start,
            end,
            i: 0,
            max_i: max((end[0] - start[0]).abs(), (end[1] - start[1]).abs()),
            delta: [get_delta(start[0], end[0]), get_delta(start[1], end[1])],
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.i > self.max_i {
            None
        } else {
            let [dx, dy] = self.delta;

            let ret = [self.start[0] + self.i * dx, self.start[1] + self.i * dy];
            self.i += 1;
            Some(ret)
        }
    }
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn part1() {
    let input = read_input("input.txt");
    let mut counts: HashMap<Point, usize> = HashMap::new();

    for line in input
        .lines()
        .filter_map(|l| l.parse::<Line>().ok())
        .filter(|l| l.is_vertical() || l.is_horizontal())
    {
        for point in &line {
            *counts.entry(point).or_insert(0) += 1
        }
    }

    let ret = counts.values().filter(|v| **v > 1).count();
    println!("{}", ret);
}

fn part2() {
    let input = read_input("input.txt");
    let mut counts: HashMap<Point, usize> = HashMap::new();

    for line in input.lines().filter_map(|l| l.parse::<Line>().ok()) {
        for point in &line {
            *counts.entry(point).or_insert(0) += 1
        }
    }

    let ret = counts.values().filter(|v| **v > 1).count();
    println!("{}", ret);
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
    use super::*;

    #[test]
    fn test_line_iter() {
        // vertical
        let l = Line {
            start: [0, 9],
            end: [5, 9],
        };
        let expected: Vec<Point> = (0..=5).map(|x| [x, 9]).collect();
        let got: Vec<Point> = l.into_iter().collect();
        assert_eq!(got, expected);

        // horizontal
        let l = Line {
            start: [5, 0],
            end: [5, 3],
        };
        let expected: Vec<Point> = (0..=3).map(|x| [5, 0 + x]).collect();
        let got: Vec<Point> = l.into_iter().collect();
        assert_eq!(got, expected);

        // antidiagonal
        let l = Line {
            start: [3, 0],
            end: [0, 3],
        };
        let expected: Vec<Point> = (0..=3).map(|x| [3 - x, 0 + x]).collect();
        let got: Vec<Point> = l.into_iter().collect();
        assert_eq!(got, expected);

        // diagonal
        let l = Line {
            start: [0, 3],
            end: [3, 0],
        };
        let expected: Vec<Point> = (0..=3).map(|x| [0 + x, 3 - x]).collect();
        let got: Vec<Point> = l.into_iter().collect();
        assert_eq!(got, expected);
    }
}
