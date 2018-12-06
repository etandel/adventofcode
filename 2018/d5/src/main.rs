use std::env;
use std::fs;


fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut right: Vec<u8> = content.into_bytes().iter().rev().cloned().collect();
    let mut left: Vec<u8> = Vec::with_capacity(right.len());

    loop {
        match (left.pop(), right.pop()) {
            (None, Some(next)) => {
                left.push(next);
            },
            (Some(top), Some(next)) => {
                if top ^ next != 32 {
                    left.push(top);
                    left.push(next);
                }
            },

            (Some(top), None) => {
                left.push(top);
                break;
            }
            (None, None) => break,

        }

    }
    println!("{}", left.len());
}


fn part2() {
}


fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}

