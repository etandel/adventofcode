use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;


const MAX_DISTANCE: usize = 10000;


#[derive(Clone, Debug, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn distance(&self, other: &Self) -> usize {
        (other.0 as i64 - self.0 as i64).abs() as usize
        + (other.1 as i64 - self.1 as i64).abs() as usize
    }
}


impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').map(str::trim).collect();
        Ok(Point(coords[0].parse::<usize>()?, coords[1].parse::<usize>()?))
    }
}


fn read_points<'a>(raw_points: &'a str) -> impl Iterator<Item = Result<Point, ParseIntError>> + 'a {
    raw_points.lines().map(Point::from_str)
}


#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Option<usize>>,
}


impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            grid: vec![None; width * height],
        }
    }

    fn get_pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.grid[self.get_pos(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, v: Option<usize>) {
        let p = self.get_pos(x, y);
        self.grid[p] = v;
    }


    fn iter_width(&self) -> impl Iterator<Item = usize> {
            0..self.width
    }

    fn iter_height(&self) -> impl Iterator<Item = usize> {
            0..self.height
    }
}


fn voronoi(grid: &mut Grid, points: &Vec<Point>) {
    for y in grid.iter_height() {
        for x in grid.iter_width() {
            let min_distance = points.iter().map(|p| Point(x, y).distance(p)).min().unwrap();
            let closest: Vec<(usize, Point)> = points
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, p)| Point(x, y).distance(p) == min_distance)
                .collect();

            if closest.len() > 1 {
                grid.set(x, y, None);
            } else {
                let pid = closest[0].0;
                grid.set(x, y, Some(pid));
            }
        }
    }
}


fn count_areas(grid: &Grid) -> BTreeMap<usize, usize> {
    let mut count = BTreeMap::new();
    for y in grid.iter_height() {
        for x in grid.iter_width() {
            if let Some(pid) = grid.get(x, y) {
                *count.entry(pid).or_insert(0) += 1;
            }
        }
    }
    count
}


fn part1(input: &str) {
    let content = fs::read_to_string(input).unwrap();
    let points: Vec<Point> = read_points(&content).collect::<Result<Vec<_>, _>>().unwrap();
    let mut grid: Grid = Grid::new(
        *points.iter().map(|Point(x, _)| x).max().unwrap() + 1,
        *points.iter().map(|Point(_, y)| y).max().unwrap() + 1
    );
    voronoi(&mut grid, &points);

    let hull: BTreeSet<usize> = grid.
        iter_height().map(|i| (0, i))
        .chain(grid.iter_height().map(|i| (grid.width - 1, i)))
        .chain(grid.iter_width().map(|i| (i, grid.height - 1)))
        .chain(grid.iter_width().map(|i| (i, 0)))
        .filter_map(|(x, y)| grid.get(x, y))
        .collect();

    let area_count = count_areas(&grid);
    let max = area_count
        .iter()
        .filter_map(|(k, v)| if hull.contains(k) {None} else {Some(v)})
        .max()
        .unwrap();

    println!("{}", max);
}


fn part2(input: &str) {
    let content = fs::read_to_string(input).unwrap();
    let points: Vec<Point> = read_points(&content).collect::<Result<Vec<_>, _>>().unwrap();
    let mut grid: Grid = Grid::new(
        *points.iter().map(|Point(x, _)| x).max().unwrap() + 1,
        *points.iter().map(|Point(_, y)| y).max().unwrap() + 1
    );

    for y in grid.iter_height() {
        for x in grid.iter_width() {
            grid.set(x, y, Some(points.iter().map(|p| Point(x, y).distance(p)).sum()))
        }
    }

    let area: usize = grid.grid.iter().cloned().filter_map(|i| i).filter(|i| *i < MAX_DISTANCE).count();
    println!("{}", area);
}


fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(args[2].as_str()),
        _ => part2(args[2].as_str()),
    };
}

