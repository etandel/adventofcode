use std::env;
use std::fs;

type Seat = (u16, u16);

fn calc_seat(raw_seat: &str) -> Seat {
    let row_range = raw_seat[0..=6].chars().fold((0.0f64, 127.0f64), |(lower, upper), c| {
        if c == 'F' {
            (lower, (lower + (upper - lower) / 2.0).floor())
        } else {
            ((lower + (upper - lower) / 2.0).ceil(), upper)
        }
    });

    let seat_range = raw_seat[7..=9].chars().fold((0f64, 7f64), |(lower, upper), c| {
        if c == 'L' {
            (lower, (lower + (upper - lower) / 2.0).floor())
        } else {
            ((lower + (upper - lower) / 2.0).ceil(), upper)
        }
    });

    (row_range.0 as u16, seat_range.0 as u16)
}

fn calc_id((row, col): Seat) -> u16 {
    row * 8 + col
}

fn part1() {
    let max = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(calc_seat)
        .map(calc_id)
        .max()
        .unwrap();
    println!("{}", max);
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
