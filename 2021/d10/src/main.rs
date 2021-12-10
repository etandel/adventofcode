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

#[derive(Debug)]
enum Corruption {
    Invalid(char),
    Missing(Vec<char>),
}

fn score_of(c: char) -> Score {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid char: {}", c),
    }
}

fn get_expected(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid opening char: {}", c),
    }
}

fn get_problem(line: &str) -> Option<Corruption> {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let top = stack.pop().unwrap();
            if c != get_expected(top) {
                return Some(Corruption::Invalid(c));
            }
        }
    }

    if stack.is_empty() {
        None
    } else {
        let mut missing = Vec::with_capacity(stack.len());

        while let Some(open) = stack.pop() {
            missing.push(get_expected(open));
        }

        Some(Corruption::Missing(missing))
    }
}

fn part1() {
    let lines = read_lines("input.txt");
    let mut score = 0;
    for line in lines {
        if let Some(Corruption::Invalid(c)) = get_problem(&line) {
            score += score_of(c);
        }
    }

    println!("{}", score);
}

fn part2() {
    let lines = read_lines("input.txt");
    let mut scores = Vec::with_capacity(lines.len());
    for line in lines {
        if let Some(Corruption::Missing(missing)) = get_problem(&line) {
            let mut score: Score = 0;
            for c in missing {
                score = score.saturating_mul(5).saturating_add(match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Invalid missing: {}", c),
                });
            }

            scores.push(score);
        }
    }

    scores.sort();

    println!("{}", scores[scores.len() / 2]);
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
