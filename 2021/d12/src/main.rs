use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::path::Path;


type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;


fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>
{
    fs::read_to_string(path)
        .unwrap()
}

fn build_graph(input: &str) -> Graph {
    let mut g = Graph::new();

    let pairs = input
        .lines()
        .map(|l| {
            let mut s = l.split('-');
            (s.next().unwrap(), s.next().unwrap())
        });

    for (from, to) in pairs {
        g.entry(from).or_insert_with(|| HashSet::new()).insert(to);
        g.entry(to).or_insert_with(|| HashSet::new()).insert(from);
    }

    g
}

fn is_small(s: &str) -> bool {
    s.chars().all(char::is_lowercase)
}

type Visited<'a> = Vec<&'a str>;

fn should_visit(visited: &Visited, n: &str) -> bool {
    !is_small(n) || !visited.contains(&n)
}

fn search<'a>(graph: &'a Graph, visited: &'a Visited<'a>, n: &'a str, mut _count: usize) -> usize {
    if n == "end" {
        1
    } else {
        let mut count = 0;

        let neighbors: Vec<&&str> = graph.get(n).iter().flat_map(|&v| v.iter()).collect();
        for neighbor in neighbors {
            if should_visit(visited, neighbor) {
                let mut subvisited = visited.clone();
                subvisited.push(neighbor);
                count += search(graph, &subvisited, neighbor, count);
            }
        }

        count
    }
}

fn part1() {
    let input = read_input("input.txt");
    let graph = build_graph(&input);

    let mut visited = Visited::new();
    visited.push("start");
    let count = search(&graph, &visited, "start", 0);

    println!("{}", count);
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
