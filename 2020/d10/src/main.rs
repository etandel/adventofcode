use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use ndarray::{Array, Dim};

type Adapter = u64;

fn parse_adapters<P>(path: P) -> Vec<Adapter>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Adapter::from_str(l).unwrap())
        .sorted()
        .collect()
}

fn part1() {
    let adapters = parse_adapters("input.txt");

    let counts = (0..1)
        .chain(adapters.iter().copied())
        .zip(adapters.iter().copied())
        .map(|(x, y)| y - x)
        .counts();

    let res = counts.get(&1).unwrap() * (1 + counts.get(&3).unwrap());
    println!("{}", res);
}

fn build_adjancy_matrix(nodes: &Vec<Adapter>) -> Array<Adapter, Dim<[usize; 2]>> {
    let n = nodes.len();
    let mut m = Array::zeros((n, n));
    let indexed: Vec<(usize, Adapter)> = nodes.iter().copied().enumerate().collect();

    for (i, from) in &indexed[..] {
        for (j, to) in &indexed[i + 1..] {
            if from + 3 < *to {
                break;
            }

            m[[*i, *j]] = 1;
        }
    }
    m[[n - 1, n - 1]] = 1;
    m
}

fn part2() {
    let adapters = parse_adapters("input.txt");

    let nodes: Vec<Adapter> = {
        let mut nodes = Vec::with_capacity(adapters.len() + 2);
        nodes.extend((0..1).chain(adapters.iter().copied()));
        nodes.push(nodes.last().unwrap() + 3);
        nodes
    };

    let mut m = build_adjancy_matrix(&nodes);

    while {
        let m2 = m.clone();
        m = m.dot(&m);

        m2 != m
    } {}

    println!("{}", m[[0, nodes.len() - 1]]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}
