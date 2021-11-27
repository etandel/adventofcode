use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BagCount {
    name: String,
    count: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Rule {
    bag_name: String,
    contains: Vec<BagCount>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.+)$").unwrap();
            static ref CONTAIN_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?[,.]").unwrap();
        }
        let captures = NAME_RE.captures(line).unwrap();
        let bag_name = captures.get(1).unwrap().as_str().to_string();
        let raw_contain = captures.get(2).unwrap();

        let contains = CONTAIN_RE
            .captures_iter(raw_contain.as_str())
            .map(|c| BagCount {
                name: c.get(2).unwrap().as_str().into(),
                count: u64::from_str(c.get(1).unwrap().as_str()).unwrap(),
            })
            .collect();

        Ok(Self { bag_name, contains })
    }
}

type RuleSet = Vec<Rule>;

#[derive(Debug, Default)]
struct Dependencies(HashMap<String, HashSet<BagCount>>);

impl Dependencies {
    fn new() -> Self {
        Self::default()
    }

    fn push_dep(&mut self, contained: BagCount, bag_name: String) {
        let depends_on = self.0.entry(bag_name).or_default();
        (*depends_on).insert(contained);
    }

    fn sum_sub_bags(&self, from: String) -> u64 {
        let mut to_see_queue = Vec::with_capacity(self.0.len());

        to_see_queue.push((1, from));

        let mut total = 0;
        while let Some((parent_count, next)) = to_see_queue.pop() {
            total += parent_count;
            if let Some(bags) = self.0.get(&next) {
                for bag in bags {
                    to_see_queue.push((parent_count * bag.count, bag.name.clone()));
                }
            }
        }

        total - 1
    }
}

impl From<RuleSet> for Dependencies {
    fn from(ruleset: RuleSet) -> Self {
        let mut deps = Self::new();

        for rule in ruleset {
            let bag_name = rule.bag_name;
            for contain in rule.contains {
                deps.push_dep(contain, bag_name.clone());
            }
        }

        deps
    }
}

#[derive(Debug, Default)]
struct ReverseDependencies(HashMap<String, HashSet<String>>);

impl ReverseDependencies {
    fn new() -> Self {
        Self::default()
    }

    fn push_dep(&mut self, contained: &BagCount, bag_name: String) {
        let depends_on = self.0.entry(contained.name.clone()).or_default();
        (*depends_on).insert(bag_name);
    }

    fn count_reachable_from(&self, from: String) -> usize {
        let mut seen: HashSet<&str> = HashSet::with_capacity(self.0.len());
        let mut to_see_queue = Vec::with_capacity(self.0.len());

        to_see_queue.push(from);

        while let Some(next) = to_see_queue.pop() {
            if let Some(bags) = self.0.get(&next) {
                for bag in bags {
                    if !seen.contains(&bag[..]) {
                        to_see_queue.push(bag.to_string());
                        seen.insert(bag);
                    }
                }
            }
        }

        seen.len()
    }
}

impl From<RuleSet> for ReverseDependencies {
    fn from(ruleset: RuleSet) -> Self {
        let mut deps = Self::new();

        for rule in ruleset {
            let bag_name = rule.bag_name;

            for contain in &rule.contains {
                deps.push_dep(contain, bag_name.clone());
            }
        }

        deps
    }
}

fn parse_rule_set<P>(path: P) -> RuleSet
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Rule::from_str(l).unwrap())
        .collect::<RuleSet>()
}

fn part1() {
    let res = ReverseDependencies::from(parse_rule_set("input.txt"))
        .count_reachable_from("shiny gold".into());

    println!("{}", res);
}

fn part2() {
    let res = Dependencies::from(parse_rule_set("input.txt")).sum_sub_bags("shiny gold".into());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_from_str() {
        let s = "dotted black bags contain no other bags.";
        let expected = Rule {
            bag_name: "dotted black".into(),
            contains: Vec::new(),
        };
        assert!(Rule::from_str(s) == Ok(expected));

        let s = "bright white bags contain 1 shiny gold bag.";
        let expected = Rule {
            bag_name: "bright white".into(),
            contains: vec![BagCount {
                name: "shiny gold".into(),
                count: 1,
            }],
        };
        assert!(Rule::from_str(s) == Ok(expected));

        let s = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";
        let expected = Rule {
            bag_name: "muted yellow".into(),
            contains: vec![
                BagCount {
                    name: "shiny gold".into(),
                    count: 2,
                },
                BagCount {
                    name: "faded blue".into(),
                    count: 9,
                },
            ],
        };
        assert!(Rule::from_str(s) == Ok(expected));
    }
}
