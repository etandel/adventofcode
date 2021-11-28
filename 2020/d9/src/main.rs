use std::collections::VecDeque;
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

fn preamble_allows(preamble: &Preamble, value: Packet) -> bool {
    preamble
        .iter()
        .combinations(2)
        .map(|v| v.iter().copied().sum())
        .find(|&s: &Packet| s == value)
        .is_some()
}

fn find_invalid_number(stream: &Stream, preamble_size: usize) -> Option<Packet> {
    let mut preamble: VecDeque<Packet> = stream.iter().take(preamble_size).copied().collect();

    let mut found: Option<Packet> = None;

    for &packet in stream.iter().skip(preamble_size) {
        if !preamble_allows(&preamble, packet) {
            found = Some(packet);
            break;
        }
        preamble.pop_front();
        preamble.push_back(packet)
    }

    found
}

fn part1() {
    let invalid = find_invalid_number(&parse_stream("input.txt"), 25);
    println!("{}", invalid.unwrap());
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
