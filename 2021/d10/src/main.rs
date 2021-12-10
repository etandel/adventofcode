use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

fn read_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.into())
        .collect()
}

type Score = u64;

fn score_of(c: char) -> Score {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid char: {}", c),
    }
}

fn get_score(line: &str) -> Score {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let top = stack.pop().unwrap();
            if let Some(score) = match (top, c) {
                ('(', ')') => None,
                ('[', ']') => None,
                ('{', '}') => None,
                ('<', '>') => None,

                (_, ')') => Some(3),
                (_, ']') => Some(57),
                (_, '}') => Some(1197),
                (_, '>') => Some(25137),
                _ => panic!("Invalid pair: ({}, {})", top, c),
            } {
                return score;
            }
        }
    }

    0
}

fn part1() {
    let lines = read_lines("input.txt");
    let mut score = 0;
    for line in lines {
        score += get_score(&line);
    }

    println!("{}", score);
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
    #[test]
    fn test_() {}
}
