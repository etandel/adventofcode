use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;

type Adapter = u64;

fn parse_adapters<P>(path: P) -> Vec<Adapter>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Adapter::from_str(l).unwrap())
        .sorted()
        .collect()
}

fn part1() {
    let adapters = parse_adapters("input.txt");

    let counts = (0..1)
        .chain(adapters.iter().copied())
        .zip(adapters.iter().copied())
        .map(|(x, y)| y - x)
        .counts();

    let res = counts.get(&1).unwrap() * (1 + counts.get(&3).unwrap());
    println!("{}", res);
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
