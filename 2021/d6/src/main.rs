use std::env;
use std::fs;
use std::path::Path;

type Timer = usize;

type Count = u64;

const MAX_TIMER: Timer = 8;

type Counts = [Count; MAX_TIMER + 1];

fn read_counts<P>(path: P) -> Counts
where
    P: AsRef<Path>,
{
    let mut counts = [0; MAX_TIMER + 1];

    for raw_timer in fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|l| l.split(','))
    {
        counts[raw_timer.parse::<Timer>().unwrap()] += 1;
    }

    counts
}

fn iter_day(before: Counts) -> Counts {
    let mut after = [0; MAX_TIMER + 1];

    for timer in 1..=MAX_TIMER {
        after[timer - 1] = before[timer];
    }

    after[MAX_TIMER] = before[0];
    after[6] += before[0];

    after
}

fn part1() {
    const NDAYS: usize = 80;

    let mut counts = read_counts("input.txt");

    for _ in 0..NDAYS {
        counts = iter_day(counts)
    }

    let res: Count = counts.iter().sum();
    println!("{}", res);
}

fn part2() {
    const NDAYS: usize = 256;

    let mut counts = read_counts("input.txt");

    for _ in 0..NDAYS {
        counts = iter_day(counts)
    }

    let res: Count = counts.iter().sum();
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
    #[test]
    fn test_() {}
}
