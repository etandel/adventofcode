use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
struct Group {
    nlines: usize,
    questions: HashMap<char, usize>,
}

impl Group {
    fn new() -> Self {
        Self::default()
    }

    fn add_line(&mut self, line: &str) {
        self.nlines += 1;
        for c in line.chars() {
            let count = self.questions.entry(c).or_insert(0);
            *count += 1;
        }
    }

    fn n_total_questions(&self) -> usize {
        self.questions.len()
    }

    fn n_common_questions(&self) -> usize {
        self.questions
            .values()
            .filter(|&v| v == &self.nlines)
            .count()
    }
}

fn parse_groups<P>(path: P) -> Vec<Group>
where
    P: AsRef<Path>,
{
    let mut groups = vec![Group::new()];
    for line in fs::read_to_string(path).unwrap().lines() {
        if line == "" {
            groups.push(Group::new());
        } else {
            groups.last_mut().unwrap().add_line(line);
        }
    }

    groups
}

fn part1() {
    let groups = parse_groups("input.txt");
    let res: usize = groups.iter().map(Group::n_total_questions).sum();
    println!("{}", res);
}

fn part2() {
    let groups = parse_groups("input.txt");
    let res: usize = groups.iter().map(Group::n_common_questions).sum();
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
