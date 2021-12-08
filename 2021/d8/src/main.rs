use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;

type Digit = u8;

lazy_static! {
    static ref POSITION_BY_SEGMENT: HashMap<Segment, usize> = ('a'..='g').zip(0..=6).collect();
    static ref DIGIT_SEGMENTS: HashMap<Digit, HashSet<Segment>> = {
        let mut m = HashMap::new();

        m.insert(0, HashSet::from_iter(['a', 'b', 'c', 'e', 'f', 'g']));
        m.insert(1, HashSet::from_iter(['c', 'f']));
        m.insert(2, HashSet::from_iter(['a', 'c', 'd', 'e', 'g']));
        m.insert(3, HashSet::from_iter(['a', 'c', 'd', 'f', 'g']));
        m.insert(4, HashSet::from_iter(['b', 'c', 'd', 'f']));
        m.insert(5, HashSet::from_iter(['a', 'b', 'd', 'f', 'g']));
        m.insert(6, HashSet::from_iter(['a', 'b', 'd', 'e', 'f', 'g']));
        m.insert(7, HashSet::from_iter(['a', 'c', 'f']));
        m.insert(8, HashSet::from_iter(['a', 'b', 'c', 'd', 'e', 'f', 'g']));
        m.insert(9, HashSet::from_iter(['a', 'b', 'c', 'd', 'f', 'g']));

        m
    };
    static ref DIGIT_POSITIONS: HashMap<Digit, HashSet<usize>> = {
        DIGIT_SEGMENTS
            .iter()
            .map(|(&digit, segments)| {
                let positions = segments
                    .iter()
                    .map(|seg| POSITION_BY_SEGMENT.get(seg).unwrap())
                    .copied()
                    .collect();
                (digit, positions)
            })
            .collect()
    };
    static ref DIGIT_SEGMENT_COUNTS: HashMap<Digit, usize> = {
        DIGIT_POSITIONS
            .iter()
            .map(|(&digit, positions)| (digit, positions.len()))
            .collect()
    };
    static ref DIGITS_BY_SEGMENT_COUNT: HashMap<usize, Vec<Digit>> = {
        let mut m = HashMap::with_capacity(10);

        for (&digit, &count) in DIGIT_SEGMENT_COUNTS.iter() {
            m.entry(count).or_insert_with(|| Vec::new()).push(digit);
        }

        m
    };
}

type Signal = String;
type Segment = char;

#[derive(Debug)]
struct Entry {
    inputs: Vec<Signal>,
    outputs: Vec<Signal>,
}

fn parse_signal_list(s: &str) -> Vec<Signal> {
    s.split_ascii_whitespace().map(|s| s.to_string()).collect()
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts: Vec<_> = line.split(" | ").map(parse_signal_list).collect();

        Ok(Self {
            outputs: parts.pop().unwrap(),
            inputs: parts.pop().unwrap(),
        })
    }
}

fn is_unique(s: &Signal) -> bool {
    let l = s.len();
    l == 2 || l == 4 || l == 3 || l == 7
}

fn read_entries<P>(path: P) -> Vec<Entry>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1() {
    let entries = read_entries("input.txt");
    let res: usize = entries
        .iter()
        .map(|e| e.outputs.iter().filter(|e| is_unique(e)).count())
        .sum();

    println!("{}", res);
}

fn get_digit(mapping: &HashMap<char, char>, wrong: &str) -> Option<u32> {
    let fixed: HashSet<_> = wrong.chars().map(|c| mapping[&c]).collect();

    for (digit, segment) in DIGIT_SEGMENTS.iter() {
        if segment == &fixed {
            return Some(*digit as u32);
        }
    }

    None
}

fn part2() {
    let entries = read_entries("input.txt");

    let mut sum = 0;

    for entry in entries {
        let mut possibilities: Vec<HashSet<Segment>> =
            { (0..7).map(|_| ('a'..='g').collect()).collect() };

        let unique: Vec<_> = entry
            .inputs
            .iter()
            .chain(entry.outputs.iter())
            .filter(|s| is_unique(s))
            .collect();

        for signal in unique {
            let segments: HashSet<Segment> = signal.chars().collect();
            let possible_digits = &DIGITS_BY_SEGMENT_COUNT[&signal.len()];

            for digit in possible_digits {
                let digit_positions = &DIGIT_POSITIONS[&digit];

                for &position in digit_positions {
                    possibilities[position] = possibilities[position]
                        .intersection(&segments)
                        .copied()
                        .collect();
                }
            }
        }

        let all_combinations: Vec<_> = possibilities[0]
            .iter()
            .cartesian_product(possibilities[1].iter())
            .cartesian_product(possibilities[2].iter())
            .cartesian_product(possibilities[3].iter())
            .cartesian_product(possibilities[4].iter())
            .cartesian_product(possibilities[5].iter())
            .cartesian_product(possibilities[6].iter())
            .map(|((((((a, b), c), d), e), f), g)| [a, b, c, d, e, f, g])
            .filter(|p| {
                let h: HashSet<_> = HashSet::from_iter(p.iter());
                h.len() == 7
            })
            .collect();

        for comb in all_combinations {
            let mapping: HashMap<char, char> =
                comb.iter().copied().copied().zip('a'..='g').collect();

            let all_converted: Vec<_> = entry
                .inputs
                .iter()
                .chain(entry.outputs.iter())
                .filter_map(|w| get_digit(&mapping, w))
                .collect();
            let l = all_converted.len();
            if l == 14 {
                let mut s = 0;

                for (i, digit) in all_converted[l - 4..l].iter().enumerate() {
                    s += *digit * 10u32.pow(3u32 - (i as u32));
                }

                sum += s;
            }
        }
    }

    println!("{}", sum);
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
