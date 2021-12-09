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

fn part1() {
    let grid = read_map("input.txt");

    let mut total_risk = 0;

    for y in 0..grid.len() {
        let row = &grid[y];

        for x in 0..row.len() {
            let candidate = grid[y][x];
            let mut adjacent = get_adjacent_heights(&grid, (y, x));
            if adjacent.all(|h| h > candidate) {
                total_risk += 1 + candidate
            }
        }
    }

    println!("{}", total_risk);
}

fn part2() {
    todo!()
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
    fn test_fueld_2() {}
}
