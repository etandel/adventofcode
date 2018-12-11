use std::collections::HashSet;
use std::env;
use std::fs;

fn react(polymer: &[u8]) -> usize {
    let mut right: Vec<u8> = polymer.to_vec();
    let mut left: Vec<u8> = Vec::with_capacity(right.len());

    loop {
        match (left.pop(), right.pop()) {
            (None, Some(next)) => {
                left.push(next);
            }
            (Some(top), Some(next)) => {
                if top ^ next != 32 {
                    left.push(top);
                    left.push(next);
                }
            }

            (Some(top), None) => {
                left.push(top);
                break;
            }
            (None, None) => break,
        }
    }
    left.len()
}

fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("{}", react(&content.bytes().collect::<Vec<u8>>()));
}

fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let bytes: Vec<u8> = content.bytes().collect();
    let candidates: HashSet<u8> = content.as_str().to_lowercase().bytes().collect();

    let min = candidates
        .iter()
        .map(|candidate| {
            react(
                &bytes
                    .iter()
                    .cloned()
                    .filter(|c| c != candidate && *c != candidate ^ 32)
                    .collect::<Vec<u8>>(),
            )
        })
        .min()
        .unwrap();
    println!("{}", min);
}

fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}
