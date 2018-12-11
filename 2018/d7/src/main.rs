extern crate regex;

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::env;
use std::fs;

use regex::Regex;

static RULE_REGEX: &str = "Step (.) must be finished before step (.) can begin.";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct MinChar(u8);

impl PartialOrd for MinChar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0).reverse())
    }
}

impl Ord for MinChar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Debug)]
struct Rule(u8, u8);

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules
        .lines()
        .map(|line| {
            let caps = Regex::new(RULE_REGEX).unwrap().captures(line).unwrap();
            Rule(caps[1].as_bytes()[0], caps[2].as_bytes()[0])
        })
        .collect()
}

#[derive(Clone, Debug)]
struct CharGraph {
    edges: BTreeMap<u8, BTreeSet<u8>>,
    rev_edges: BTreeMap<u8, BTreeSet<u8>>,
}

impl CharGraph {
    fn new() -> CharGraph {
        CharGraph {
            edges: BTreeMap::new(),
            rev_edges: BTreeMap::new(),
        }
    }

    fn from_rules<'a, I>(rules: I) -> CharGraph
    where
        I: Iterator<Item = &'a Rule>,
    {
        let mut g = CharGraph::new();
        for rule in rules {
            g.add_edge(rule);
        }
        g
    }

    fn add_edge(&mut self, rule: &Rule) {
        let Rule(from, to) = rule;
        (*self.edges.entry(*from).or_insert(BTreeSet::new())).insert(*to);
        (*self.rev_edges.entry(*to).or_insert(BTreeSet::new())).insert(*from);
    }

    fn remove_edge(&mut self, from: &u8, to: &u8) {
        self.edges.get_mut(from).unwrap().remove(to);
        self.rev_edges.get_mut(to).unwrap().remove(from);
    }

    fn get_children(&self, node: &u8) -> BTreeSet<u8> {
        self.edges
            .get(node)
            .map_or(BTreeSet::new(), |edges| edges.clone())
    }

    fn get_parents(&self, node: &u8) -> BTreeSet<u8> {
        self.rev_edges
            .get(node)
            .map_or(BTreeSet::new(), |edges| edges.clone())
    }

    fn is_root(&self, c: &u8) -> bool {
        self.rev_edges.get(c).map_or(false, BTreeSet::is_empty)
    }

    fn find_roots(&self) -> Vec<u8> {
        let with_children: BTreeSet<u8> = self.edges.keys().cloned().collect();
        let with_parents: BTreeSet<u8> = self.rev_edges.keys().cloned().collect();
        with_children.difference(&with_parents).cloned().collect()
    }

    fn node_count(&self) -> usize {
        self.edges.keys().len()
    }

    fn toposort(&self) -> Vec<u8> {
        let mut graph = self.clone();
        let mut ordered: Vec<u8> = Vec::with_capacity(self.node_count());

        let mut to_visit: BinaryHeap<MinChar> =
            graph.find_roots().iter().map(|c| MinChar(*c)).collect();
        loop {
            if let Some(MinChar(next)) = to_visit.pop() {
                for to in graph.get_children(&next).iter() {
                    graph.remove_edge(&next, to);
                    if graph.is_root(to) {
                        to_visit.push(MinChar(*to));
                    }
                }
                ordered.push(next);
            } else {
                break;
            }
        }
        ordered
    }
}

fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let rules = parse_rules(&content);

    let graph = CharGraph::from_rules(rules.iter());
    println!("{}", String::from_utf8(graph.toposort()).unwrap());
}

fn get_cost(c: u8) -> usize {
    (c - b'A' + 1 + 60) as usize
}

fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let rules = parse_rules(&content);

    let graph = CharGraph::from_rules(rules.iter());
    let sorted = graph.toposort();

    let mut workers: Vec<Option<(u8, usize)>> = vec![None; 5];
    let mut total_time: usize = 0;
    let mut done: BTreeSet<u8> = BTreeSet::new();
    let mut assigned: BTreeSet<u8> = BTreeSet::new();

    loop {
        let mut next_possible: Vec<u8> = sorted
            .iter()
            .cloned()
            .filter(|j| {
                !done.contains(j)
                    && !assigned.contains(j)
                    && graph.get_parents(j).difference(&done).next().is_none()
            })
            .collect();
        next_possible.sort_unstable_by(|a, b| a.cmp(b).reverse());

        let free_workers: Vec<usize> = workers
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, w)| if w.is_none() { Some(i) } else { None })
            .collect();

        if next_possible.len() == 0 && free_workers.len() == 5 {
            break;
        }

        for i in free_workers.iter() {
            if let Some(job) = next_possible.pop() {
                workers[*i] = Some((job, get_cost(job)));
                assigned.insert(job);
            }
        }

        let (_, tick) = workers
            .iter()
            .filter(|w| w.is_some())
            .min_by_key(|v| v.unwrap().1)
            .unwrap()
            .unwrap();
        total_time += tick;

        let assigned_workers: Vec<(usize, u8, usize)> = workers
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| match v {
                None => None,
                Some((job, w)) => Some((i, job, w)),
            })
            .collect();

        for (i, job, w) in assigned_workers {
            match w - tick {
                0 => {
                    done.insert(job);
                    workers[i] = None;
                }
                x => {
                    workers[i] = Some((job, x));
                }
            };
        }
    }

    println!("{}", total_time);
}

fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}
