use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;

type Packet = u64;
type Stream = Vec<Packet>;
type Preamble = VecDeque<Packet>;

fn parse_stream<P>(path: P) -> Stream
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Packet::from_str(l).unwrap())
        .collect()
}

fn allowed_values(preamble: &Preamble) -> HashSet<Packet> {
    preamble
        .iter()
        .combinations(2)
        .map(|v| v.iter().copied().sum())
        .collect()
}

fn part1() {
    const STREAM_SIZE: usize = 25;
    let stream = parse_stream("input.txt");

    let mut preamble: VecDeque<Packet> = stream.iter().take(STREAM_SIZE).copied().collect();

    let mut found: Option<Packet> = None;

    for &packet in stream.iter().skip(STREAM_SIZE) {
        if !allowed_values(&preamble).contains(&packet) {
            found = Some(packet);
            break;
        }
        preamble.pop_front();
        preamble.push_back(packet)
    }

    println!("{}", found.unwrap());
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
