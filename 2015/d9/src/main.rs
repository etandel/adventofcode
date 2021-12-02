use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

use itertools::Itertools;

type Node<'a> = &'a str;
type Dist = usize;

fn parse_distance(s: &str) -> ((Node, Node), Dist) {
    match s.split_ascii_whitespace().collect::<Vec<&str>>().as_slice() {
        [from, "to", to, "=", raw_dist] => ((from, to), raw_dist.parse().unwrap()),
        _ => panic!("Invalid distance: {}", s),
    }
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn distances(inp: &str) -> HashMap<(Node, Node), Dist> {
    inp.lines().map(parse_distance).collect()
}

fn get_all_nodes<'a>(distances: &'a HashMap<(Node, Node), Dist>) -> HashSet<&'a str> {
    distances
        .keys()
        .flat_map(|(n1, n2)| [n1, n2])
        .copied()
        .collect()
}

fn iter_possible_distances<'a>(
    distances: &'a HashMap<(Node, Node), Dist>,
    nodes: &'a HashSet<&'a str>,
) -> impl Iterator<Item = Dist> + 'a {
    nodes.iter().permutations(nodes.len()).map(|perm| {
        let dist = perm
            .windows(2)
            .map(|window| match window {
                [&from, &to] => distances
                    .get(&(from, to))
                    .or_else(|| distances.get(&(to, from)))
                    .unwrap(),
                _ => panic!("Invalid window!"),
            })
            .sum();
        dist
    })
}

fn part1() {
    let input = read_input("input.txt");
    let distances = distances(&input[..]);
    let nodes = get_all_nodes(&distances);
    // this is O(n!) and could probably be faster by using eulerian paths or circuits,
    // but the input has only 8 nodes, so who cares? =D
    let min_dist: Dist = iter_possible_distances(&distances, &nodes).min().unwrap();

    println!("{}", min_dist);
}

fn part2() {
    let input = read_input("input.txt");
    let distances = distances(&input[..]);
    let nodes = get_all_nodes(&distances);
    let min_dist: Dist = iter_possible_distances(&distances, &nodes).max().unwrap();

    println!("{}", min_dist);
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
