use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Signal = String;

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

fn read_entries<P>(path: P) -> Vec<Entry>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1() {
    let entries = read_entries("input.txt");
    let res: usize = entries
        .iter()
        .map(|e| {
            e.outputs
                .iter()
                .filter(|o| {
                    let l = o.len();
                    l == 2 || l == 4 || l == 3 || l == 7
                })
                .count()
        })
        .sum();

    println!("{}", res);
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
    fn test_fueld_2() {}
}
