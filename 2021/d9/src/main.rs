use std::env;
use std::fs;
use std::path::Path;

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

fn get_adjacent(grid: &Grid, (y, x): (usize, usize)) -> Vec<Height> {
    let deltas = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    deltas
        .iter()
        .filter_map(|(dy, dx)| {
            add(y, *dy)
                .iter()
                .map(|newy| add(x, *dx).map(|newx| (*newy, newx)))
                .next()
        })
        .flatten()
        .filter_map(|(y, x)| grid.get(y).iter().flat_map(|row| row.get(x)).next())
        .copied()
        .collect()
}

fn part1() {
    let map = read_map("input.txt");

    let mut total_risk = 0;

    for y in 0..map.len() {
        let row = &map[y];

        for x in 0..row.len() {
            let candidate = map[y][x];
            let adjacent = get_adjacent(&map, (y, x));
            if adjacent.iter().all(|h| *h > candidate) {
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
