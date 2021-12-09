use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

use lazy_static::lazy_static;

type Height = u16;
type Row = Vec<Height>;
type Grid = Vec<Row>;

fn read_map<P>(path: P) -> Grid
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as Height)
                .collect()
        })
        .collect()
}

fn add(t: usize, dt: i8) -> Option<usize> {
    if dt < 0 {
        t.checked_sub(dt.abs() as usize)
    } else {
        Some(t + (dt as usize))
    }
}

fn get_height((y, x): (usize, usize), grid: &Grid) -> Option<Height> {
    grid.get(y)?.get(x).map(|h| *h)
}

fn get_adjacent_positions((y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    lazy_static! {
        static ref DELTAS: [(i8, i8); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    }

    DELTAS
        .iter()
        .filter_map(move |(dy, dx)| Some((add(y, *dy)?, add(x, *dx)?)))
}

fn get_adjacent_heights<'a>(
    grid: &'a Grid,
    pos: (usize, usize),
) -> impl Iterator<Item = Height> + 'a {
    get_adjacent_positions(pos).filter_map(|pos| get_height(pos, grid))
}

fn get_lowest<'a>(grid: &'a Grid) -> impl Iterator<Item = (usize, usize)> + 'a {
    grid.iter()
        .enumerate()
        .flat_map(move |(y, row)| row.iter().enumerate().map(move |(x, _)| (y, x)))
        .filter(move |&(y, x)| {
            let candidate = grid[y][x];
            get_adjacent_heights(&grid, (y, x)).all(|h| h > candidate)
        })
}

fn part1() {
    let grid = read_map("input.txt");

    let total_risk: Height = get_lowest(&grid).map(|(y, x)| grid[y][x] + 1).sum();

    println!("{}", total_risk);
}

fn part2() {
    let grid = read_map("input.txt");

    let mut basin_sizes: HashMap<(usize, usize), usize> =
        get_lowest(&grid).map(|pos| (pos, 0)).collect();

    let seeds: Vec<(usize, usize)> = basin_sizes.keys().copied().collect();
    // assumes a 1-1 relation between seeds and basins
    for seed in seeds {
        let mut to_visit: Vec<(usize, usize)> = Vec::with_capacity(grid.len() * grid.len());
        let mut visited: HashSet<(usize, usize)> = HashSet::with_capacity(grid.len() * grid.len());

        to_visit.push(seed);

        while !to_visit.is_empty() {
            let next = to_visit.pop().unwrap();

            visited.insert(next);
            to_visit.extend(get_adjacent_positions(next).filter(|pos| {
                get_height(*pos, &grid).map(|h| h != 9).unwrap_or(false) && !visited.contains(pos)
            }));
        }

        if let Some(x) = basin_sizes.get_mut(&seed) {
            *x += visited.len()
        }
    }

    let mut sizes: Vec<_> = basin_sizes.values().collect();
    sizes.sort();

    let ret: usize = sizes[sizes.len() - 3..sizes.len()]
        .iter()
        .copied()
        .product();

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
    #[test]
    fn test_() {}
}
