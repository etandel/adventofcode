use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

type PotState = bool;

fn pot_to_u8(pot: PotState) -> u8 {
    if pot {
        b'#'
    } else {
        b'.'
    }
}

//fn pot_to_char(pot: PotState) -> char {
//    if pot {'#'} else {'.'}
//}

fn read_state(s: &str) -> impl Iterator<Item = PotState> + '_ {
    s.bytes().map(|b| b == b'#')
}

#[derive(Debug)]
struct RuleSet {
    rules: BTreeMap<Vec<PotState>, PotState>,
}

impl RuleSet {
    fn from_serialized_rules(serialized_rules: std::str::Lines) -> RuleSet {
        let mut rules = BTreeMap::new();
        for r in serialized_rules {
            let rule: Vec<&str> = r.split(' ').collect();
            let from = read_state(rule[0]).collect();
            let to = read_state(rule[2]).next().unwrap();
            rules.insert(from, to);
        }
        RuleSet { rules }
    }

    fn get(&self, state_slice: &[PotState]) -> PotState {
        self.rules[state_slice]
    }
}

#[derive(Debug)]
struct PotRow {
    pots: VecDeque<PotState>,
}

impl PotRow {
    fn tick(&self, rule_set: &RuleSet) -> Self {
        let mut new_pots = VecDeque::with_capacity(self.pots.len());

        // TODO: handle first 2 pots (assume everything to the left is '.')
        new_pots.push_back(self.pots[0]);
        new_pots.push_back(self.pots[1]);

        for i in 0..=self.pots.len() - 5 {
            // TODO optimize this O(N^2)
            let slice: Vec<PotState> = self.pots.iter().cloned().skip(i).take(5).collect();
            new_pots.push_back(rule_set.get(&slice))
        }

        // TODO: handle last 2 pots (assume everything to the right is '.')
        new_pots.push_back(self.pots[self.pots.len() - 2]);
        new_pots.push_back(self.pots[self.pots.len() - 1]);

        PotRow { pots: new_pots }
    }

    fn tick_mut(&mut self, rule_set: &RuleSet) {
        self.pots = self.tick(rule_set).pots;
    }
}

impl FromStr for PotRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PotRow {
            pots: read_state(s).collect(),
        })
    }
}

impl fmt::Display for PotRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<u8> = self.pots.iter().cloned().map(pot_to_u8).collect();
        write!(f, "{}", String::from_utf8(s).unwrap())
    }
}

fn read_initial_state(s: &str) -> PotRow {
    let serialized_state = s.split(' ').last().unwrap();
    PotRow::from_str(serialized_state).unwrap()
}

fn part1(input: &str) {
    let content = fs::read_to_string(input).unwrap();
    let mut lines = content.lines();

    let mut pot_row = read_initial_state(lines.next().unwrap());
    lines.next(); //  ignore blank line
    let rule_set = RuleSet::from_serialized_rules(lines);

    println!("{}", pot_row);
    pot_row.tick_mut(&rule_set);
    println!("{}", pot_row);

    println!("WIP");
}

fn part2(input: &str) {
    let _content = fs::read_to_string(input).unwrap();
    println!("Not implemented");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(args[2].as_str()),
        _ => part2(args[2].as_str()),
    };
}
