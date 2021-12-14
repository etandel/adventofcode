use std::cmp::{max, min};
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

fn parse_rules<'a, I>(lines: I) -> HashMap<[char; 2], char>
where
    I: Iterator<Item = &'a str>,
{
    let mut rules = HashMap::new();

    for line in lines.skip(1) {
        let mut chars = line.chars();

        let from = [chars.next().unwrap(), chars.next().unwrap()];
        // skip ' -> '
        let to = chars.skip(4).next().unwrap();

        rules.insert(from, to);
    }

    rules
}

fn print_template(t: &Vec<char>) {
    println!("{}", t.iter().collect::<String>());
}

fn part1() {
    let input = read_input("input.txt");
    let mut lines = input.lines();

    let mut template: Vec<char> = lines.by_ref().next().unwrap().chars().collect();
    let rules = parse_rules(lines);

    for _step in 0..10 {
        let mut new_template = Vec::with_capacity(template.len() * 2 - 1);

        new_template.push(template[0]);
        template.windows(2).for_each(|w| match w {
            from @ &[_, b] => {
                new_template.push(rules[from]);
                new_template.push(b);
            }
            _ => panic!("Invalid window {:?}", w),
        });

        template = new_template;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    let mut min_count = usize::MAX;
    let mut max_count = 0;

    for c in template {
        let e = counts.entry(c).or_insert(0);
        *e += 1;

        min_count = min(min_count, *e);
        max_count = max(max_count, *e);
    }

    let mut counts: Vec<_> = counts.values().collect();
    counts.sort();
    println!("{}", *counts.last().unwrap() - counts[0]);
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
