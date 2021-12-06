use std::cmp::{max, min};
use std::env;
use std::fs;
use std::path::Path;

type Pos = i64;

fn read_positions<P>(path: P) -> Vec<Pos>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn total_fuel(positions: &[Pos], dest: Pos) -> Pos {
    positions.iter().map(|p| (p - dest).abs()).sum()
}

fn fuel_2(start: Pos, dest: Pos) -> Pos {
    let from = min(start, dest);
    let to = max(start, dest);
    (from..to).enumerate().map(|(i, _)| (i + 1) as Pos).sum()
}

fn total_fuel_2(positions: &[Pos], dest: Pos) -> Pos {
    positions.iter().map(|p| fuel_2(*p, dest)).sum()
}

fn part1() {
    let positions = read_positions("input.txt");
    let res = positions
        .iter()
        .map(|x| total_fuel(&positions, *x))
        .min()
        .unwrap();

    println!("{}", res);
}

fn part2() {
    let positions = read_positions("input.txt");
    let possibilities: Vec<Pos> =
        (*positions.iter().min().unwrap()..*positions.iter().max().unwrap()).collect();
    let res = possibilities
        .iter()
        .map(|x| total_fuel_2(&positions, *x))
        .min()
        .unwrap();

    println!("{}", res);
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
    fn test_fueld_2() {
        assert_eq!(fuel_2(16, 5), 66);
        assert_eq!(fuel_2(1, 5), 10);
        assert_eq!(fuel_2(2, 5), 6);
        assert_eq!(fuel_2(0, 5), 15);
        assert_eq!(fuel_2(4, 5), 1);
        assert_eq!(fuel_2(2, 5), 6);
        assert_eq!(fuel_2(7, 5), 3);
        assert_eq!(fuel_2(1, 5), 10);
        assert_eq!(fuel_2(2, 5), 6);
        assert_eq!(fuel_2(14, 5), 45);
    }
}
