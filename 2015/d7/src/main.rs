use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Sig = u16;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Not(String, String),
    LeftShift(String, Sig, String),
    RightShift(String, Sig, String),
    Wire(String, String),
    Input(Sig, String),
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (input, output) = {
            let parts = s.split(" -> ").collect::<Vec<&str>>();
            (parts[0].to_string(), parts[1].to_string())
        };

        let gate = match input
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [w1, "AND", w2] => Gate::And(w1.to_string(), w2.to_string(), output),
            [w1, "OR", w2] => Gate::Or(w1.to_string(), w2.to_string(), output),
            ["NOT", w1] => Gate::Not(w1.to_string(), output),
            [w1, "LSHIFT", val] => {
                Gate::LeftShift(w1.to_string(), Sig::from_str(val).unwrap(), output)
            }
            [w1, "RSHIFT", val] => {
                Gate::RightShift(w1.to_string(), Sig::from_str(val).unwrap(), output)
            }
            [inp] => match Sig::from_str(inp) {
                Err(_) => Gate::Wire(inp.to_string(), output),
                Ok(sig) => Gate::Input(sig, output),
            },
            _ => panic!(),
        };
        Ok(gate)
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Circuit {
    signals: HashMap<String, Sig>,
    gates_by_output: HashMap<String, Gate>,
}

impl Circuit {
    fn with_capacity(cap: usize) -> Self {
        Self {
            signals: HashMap::with_capacity(cap),
            gates_by_output: HashMap::with_capacity(cap),
        }
    }

    fn push_gate(&mut self, gate: Gate) {
        let output = match gate {
            Gate::And(_, _, ref output) => output,
            Gate::Or(_, _, ref output) => output,
            Gate::Not(_, ref output) => output,
            Gate::LeftShift(_, _, ref output) => output,
            Gate::RightShift(_, _, ref output) => output,
            Gate::Wire(_, ref output) => output,
            Gate::Input(_, ref output) => output,
        };

        self.gates_by_output.insert(output.to_string(), gate);
    }
}

struct Sim {
    circuit: Circuit,
}

impl Sim {
    fn get_signal_for_cached<'a>(
        &'a self,
        cache: &mut HashMap<&'a String, Sig>,
        wire: &'a String,
    ) -> Sig {
        if let Some(cached) = cache.get(wire) {
            *cached
        } else {
            let got = self.circuit.gates_by_output.get(wire);
            let sig = match got {
                Some(w) => match w {
                    Gate::And(w1, w2, _) => {
                        self.get_signal_for_cached(cache, w1)
                            & self.get_signal_for_cached(cache, w2)
                    }

                    Gate::Or(w1, w2, _) => {
                        self.get_signal_for_cached(cache, w1)
                            | self.get_signal_for_cached(cache, w2)
                    }

                    Gate::Not(w1, _) => !self.get_signal_for_cached(cache, w1),

                    Gate::LeftShift(w1, val, _) => self.get_signal_for_cached(cache, w1) << val,

                    Gate::RightShift(w1, val, _) => self.get_signal_for_cached(cache, w1) >> val,

                    Gate::Wire(w1, _) => self.get_signal_for_cached(cache, w1),

                    Gate::Input(val, _) => *val,
                },
                None => Sig::from_str(wire).unwrap(),
            };
            cache.insert(&wire, sig);
            sig
        }
    }
    fn get_signal_for(&self, wire: &String) -> Sig {
        let mut cache = HashMap::with_capacity(self.circuit.gates_by_output.len());
        self.get_signal_for_cached(&mut cache, wire)
    }
}

fn parse_circuit<P>(path: P) -> Circuit
where
    P: AsRef<Path>,
{
    let data = fs::read_to_string(path).unwrap();
    let lines = data.lines().collect::<Vec<&str>>();

    let mut circuit = Circuit::with_capacity(lines.len());

    for line in lines {
        circuit.push_gate(Gate::from_str(line).unwrap())
    }

    circuit
}

fn part1() {
    let circuit = parse_circuit("input.txt");
    let sim = Sim { circuit };
    let res = sim.get_signal_for(&"a".to_string());
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
    fn test_() {}
}
