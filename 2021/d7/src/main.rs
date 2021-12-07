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

fn total_fuel<F>(positions: &[Pos], dest: Pos, metric: F) -> Pos
where
    F: Fn(Pos, Pos) -> Pos,
{
    positions.iter().map(|p| metric(*p, dest)).sum()
}

fn manhattan_metric(start: Pos, dest: Pos) -> Pos {
    (dest - start).abs()
}

fn ap_metric(start: Pos, dest: Pos) -> Pos {
    let n = (start - dest).abs();
    n * (1 + n) / 2
}

fn part1() {
    let mut positions = read_positions("input.txt");
    positions.sort();
    let median = positions[positions.len() / 2];
    let res = total_fuel(&positions, median, manhattan_metric);
    println!("{}", res);
}

fn part2() {
    let positions = read_positions("input.txt");
    let res = positions
        .iter()
        .map(|x| total_fuel(&positions, *x, ap_metric))
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
        assert_eq!(ap_metric(16, 5), 66);
        assert_eq!(ap_metric(1, 5), 10);
        assert_eq!(ap_metric(2, 5), 6);
        assert_eq!(ap_metric(0, 5), 15);
        assert_eq!(ap_metric(4, 5), 1);
        assert_eq!(ap_metric(2, 5), 6);
        assert_eq!(ap_metric(7, 5), 3);
        assert_eq!(ap_metric(1, 5), 10);
        assert_eq!(ap_metric(2, 5), 6);
        assert_eq!(ap_metric(14, 5), 45);
    }
}
