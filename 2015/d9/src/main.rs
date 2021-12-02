use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

use itertools::Itertools;

type Node = String;
type Dist = usize;

fn parse_distance(s: &str) -> ((Node, Node), Dist) {
    match s.split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
        [from, "to", to, "=", raw_dist] => (
            (from.to_string(), to.to_string()),
            raw_dist.parse().unwrap(),
        ),
        _ => panic!("Invalid distance: {}", s),
    }
}

fn parse_distances<P>(path: P) -> HashMap<(Node, Node), Dist>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_distance)
        .collect()
}

fn part1() {
    let distances = parse_distances("input.txt");
    let nodes: HashSet<_> = distances.keys().flat_map(|(n1, n2)| [n1, n2]).collect();

    let min_dist: Dist = nodes
        .iter()
        .permutations(nodes.len())
        .map(|perm| {
            let dist = perm
                .windows(2)
                .map(|window| match window {
                    [from, to] => distances
                        .get(&(from.to_string(), to.to_string()))
                        .or_else(|| distances.get(&(to.to_string(), from.to_string())))
                        .unwrap(),
                    _ => panic!("Invalid window!"),
                })
                .sum();
            dist
        })
        .min()
        .unwrap();

    println!("{}", min_dist);
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
