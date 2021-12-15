use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::ops::Add;
use std::ops::Index;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    y: i32,
    x: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Point {
    fn new(y: i32, x: i32) -> Self {
        Self { x, y }
    }

    fn checked_add(self, rhs: Self, nrows: usize, ncols: usize) -> Option<Self> {
        let new = self + rhs;

        if new.y < 0 || new.y >= nrows as i32 || new.x < 0 || new.x >= ncols as i32 {
            None
        } else {
            Some(new)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid<T> {
    grid: Vec<T>,
    ncols: usize,
    nrows: usize,
}

impl<T> Grid<T> {
    fn neighbors_idx_4(&self, pos: Point) -> Vec<Point> {
        let deltas = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ];

        deltas
            .iter()
            .filter_map(|d| pos.checked_add(*d, self.nrows, self.ncols))
            .collect()
    }

    fn top_left(&self) -> Point {
        Point::new(0, 0)
    }

    fn bottom_right(&self) -> Point {
        Point::new((self.nrows - 1) as i32, (self.ncols - 1) as i32)
    }

    /// Starts at top-left (0, 0) and iterates over all points until bottom right (nrows, ncols)
    fn iter_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (0..self.nrows).flat_map(|y| (0..self.ncols).map(move |x| Point::new(y as i32, x as i32)))
    }

    fn get<'a>(&'a self, Point { y, x }: &Point) -> Option<&'a T> {
        self.grid.get((y * self.ncols as i32 + x) as usize)
    }

    fn from_lines<I, SubI>(mut lines: I) -> Self
    where
        I: Iterator<Item = SubI>,
        SubI: IntoIterator<Item = T>,
    {
        let mut grid: Vec<T> = Vec::new();

        let mut ncols = 0;
        for iterrow in lines.by_ref() {
            for item in iterrow {
                ncols += 1;
                grid.push(item);
            }
            break;
        }

        grid.extend(lines.flatten());

        Self {
            nrows: grid.len() / ncols,
            ncols,
            grid,
        }
    }

    fn from_string_with<'a, F, SubI>(s: &'a str, parser: &'a F) -> Self
    where
        F: Fn(&'a str) -> SubI,
        SubI: IntoIterator<Item = T> + 'a,
    {
        Self::from_lines(s.lines().map(parser))
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.index(&index)
    }
}

type Risk = u32;

fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = Risk> + 'a {
    line.chars().map(|c| c.to_digit(10).unwrap())
}

fn read_grid<P>(path: P) -> Grid<Risk>
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(path).unwrap();
    Grid::from_string_with(&s, &parse_line)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    point: Point,
    total_risk: Risk,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_risk.cmp(&self.total_risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors_idx_4_part2<T>(grid: &Grid<T>, point: Point) -> Vec<Point> {
    let deltas = [
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
    ];

    deltas
        .iter()
        .filter_map(|d| point.checked_add(*d, grid.nrows * 5, grid.ncols * 5))
        .collect()
}

fn project(dim: i32, max: usize) -> (i32, i32) {
    let projected = dim % max as i32;
    (projected, dim / max as i32)
}

fn get_risk_part_2(grid: &Grid<Risk>, Point { y, x }: Point) -> Risk {
    let (projy, dy) = project(y, grid.nrows);
    let (projx, dx) = project(x, grid.ncols);

    let delta = dy + dx;

    let risk = grid[Point::new(projy, projx)] + delta as Risk;

    if risk > 9 {
        risk % 10 + 1
    } else {
        risk
    }
}

fn dijkstra<C, N, I>(
    grid: &Grid<Risk>,
    start: Point,
    end: Point,
    get_cost: C,
    get_neighbors: N,
) -> Option<HashMap<Point, Risk>>
where
    C: Fn(&Grid<Risk>, Point) -> Risk,
    N: Fn(&Grid<Risk>, Point) -> I,
    I: IntoIterator<Item = Point>,
{
    let mut distances: HashMap<Point, Risk> = HashMap::new();

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut enqueued: HashSet<Point> = HashSet::new();

    distances.insert(start, 0);
    queue.push(Node {
        point: start,
        total_risk: 0,
    });
    enqueued.insert(start);

    while let Some(Node { point, total_risk }) = queue.pop() {
        if point == end {
            return Some(distances);
        }

        if total_risk > distances[&point] {
            continue;
        }

        for neighbor in get_neighbors(grid, point) {
            let new_risk = total_risk + get_cost(grid, neighbor);
            let current_risk = distances.entry(neighbor).or_insert(Risk::MAX);
            if new_risk < *current_risk && !enqueued.contains(&neighbor) {
                queue.push(Node {
                    point: neighbor,
                    total_risk: new_risk,
                });
                distances.insert(neighbor, new_risk);
            }
        }
    }

    None
}

fn part1() {
    let grid = read_grid("input.txt");
    let distances = dijkstra(
        &grid,
        grid.top_left(),
        grid.bottom_right(),
        |g, p| g[p],
        |g, p| g.neighbors_idx_4(p),
    )
    .unwrap();
    println!("{}", distances[&grid.bottom_right()]);
}

fn part2() {
    let grid = read_grid("input.txt");
    let end = Point::new((&grid.nrows * 5 - 1) as i32, (&grid.ncols * 5 - 1) as i32);
    let distances = dijkstra(
        &grid,
        grid.top_left(),
        end,
        get_risk_part_2,
        neighbors_idx_4_part2,
    )
    .unwrap();
    println!("{}", distances[&end]);
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
