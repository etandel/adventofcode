use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

type Pair = [char; 2];
type Transitions = HashMap<Pair, Vec<Pair>>;
const EPS: char = '!';

fn parse_transitions<'a, I>(lines: I) -> Transitions
where
    I: Iterator<Item = &'a str>,
{
    let mut rules = HashMap::new();

    for line in lines.skip(1) {
        let mut chars = line.chars();

        let from1 = chars.next().unwrap();
        let from2 = chars.next().unwrap();
        // skip ' -> '
        let to = chars.skip(4).next().unwrap();

        rules.insert([from1, from2], vec![[from1, to], [to, from2]]);

        rules.insert([EPS, from1], vec![[EPS, from1]]);
        rules.insert([from2, EPS], vec![[from2, EPS]]);
    }

    rules
}

fn solve<P>(path: P, n_iterations: usize) -> usize
where
    P: AsRef<Path>,
{
    let input = read_input(path);
    let mut lines = input.lines();

    let template: Vec<char> = lines.by_ref().next().unwrap().chars().collect();
    let transitions = parse_transitions(lines);

    let mut pair_counts: HashMap<Pair, usize> = HashMap::new();
    pair_counts.insert([EPS, *template.first().unwrap()], 1);
    pair_counts.insert([*template.last().unwrap(), EPS], 1);
    for pair in template.windows(2) {
        match pair {
            &[from1, from2] => {
                *pair_counts.entry([from1, from2]).or_insert(0) += 1;
            }
            _ => panic!("Invalid pair: {:#?}", pair),
        }
    }

    for _step in 0..n_iterations {
        let mut new_counts = HashMap::with_capacity(pair_counts.len());

        for (pair, count) in pair_counts.iter() {
            for newpair in &transitions[pair] {
                *new_counts.entry(*newpair).or_insert(0) += count;
            }
        }

        pair_counts = new_counts;
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pair_counts.iter() {
        for &c in pair {
            if c != EPS {
                *char_counts.entry(c).or_insert(0) += count;
            }
        }
    }

    let mut counts: Vec<_> = char_counts.values().collect();
    counts.sort();
    return (*counts.last().unwrap() - counts[0]) / 2;
}

fn part1() {
    println!("{}", solve("input.txt", 10));
}

fn part2() {
    println!("{}", solve("input.txt", 40));
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
