use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

type Seat = (u16, u16);

fn iterate(seq: &str, lower_char: char, upper_bound: f64) -> u16 {
    let range = seq
        .chars()
        .fold((0.0f64, upper_bound), |(lower, upper), c| {
            if c == lower_char {
                (lower, (lower + (upper - lower) / 2.0).floor())
            } else {
                ((lower + (upper - lower) / 2.0).ceil(), upper)
            }
        });

    range.0 as u16
}

fn calc_seat(raw_seat: &str) -> Seat {
    (
        iterate(&raw_seat[0..=6], 'F', 127.0f64),
        iterate(&raw_seat[7..=9], 'L', 7.0f64),
    )
}

fn calc_id((row, col): Seat) -> u16 {
    row * 8 + col
}

fn parse_ids<P>(path: P) -> Vec<u16>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(calc_seat)
        .map(calc_id)
        .collect()
}

fn part1() {
    let max = parse_ids("input.txt").iter().max().unwrap().to_owned();
    println!("{}", max);
}

fn part2() {
    let ids: HashSet<_> = parse_ids("input.txt").iter().copied().collect();
    let expected_ids: HashSet<u16> = (1..=127 * 8 + 7).collect();

    let mut missing: Vec<u16> = expected_ids.difference(&ids).copied().collect();

    missing.sort();

    let got = missing
        .iter()
        .zip(missing.iter().skip(1))
        .skip_while(|(&x1, &x2)| x2 == x1 + 1)
        .next()
        .unwrap()
        .1;

    println!("{}", got);
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
    fn test_calc_seat() {
        dbg!(calc_seat("FBFBBFFRLR"));
        assert!(calc_seat("FBFBBFFRLR") == (44, 5));
        assert!(calc_seat("BFFFBBFRRR") == (70, 7));
        assert!(calc_seat("FFFBBBFRRR") == (14, 7));
        assert!(calc_seat("BBFFBBFRLL") == (102, 4));
    }

    #[test]
    fn test_calc_id() {
        assert!(calc_id((44, 5)) == 357);
        assert!(calc_id((70, 7)) == 567);
        assert!(calc_id((14, 7)) == 119);
        assert!(calc_id((102, 4)) == 820);
    }
}
