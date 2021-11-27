use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut iter_parts = line.split_ascii_whitespace();
        let instruction_name = iter_parts.next().unwrap();
        let raw_val = iter_parts.next().unwrap();

        let val = raw_val.parse::<i64>().unwrap();
        Ok(match instruction_name {
            "nop" => Instruction::Nop(val),
            "acc" => Instruction::Acc(val),
            "jmp" => Instruction::Jmp(val),
            _ => panic!("Unexpected instruction: {}", instruction_name),
        })
    }
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Self(
            s.lines()
                .map(|l| Instruction::from_str(l).unwrap())
                .collect(),
        ))
    }
}

impl Program {
    fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<'_, Instruction> {
        self.0.iter()
    }

    fn toggle_instruction(&self, instruction: usize) -> Self {
        let mut new_vec = self.0.clone();

        new_vec[instruction] = match new_vec[instruction] {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            x => x,
        };

        Self(new_vec)
    }

    fn execute(&self) -> (i64, bool) {
        let mut seen: HashSet<i64> = HashSet::with_capacity(self.len());

        let mut acc = 0;
        let mut instruction_pointer = 0i64;

        loop {
            if seen.contains(&instruction_pointer) {
                return (acc, false);
            } else if instruction_pointer == self.len() as i64 {
                return (acc, true);
            } else if instruction_pointer > self.len() as i64 || instruction_pointer < 0 {
                return (acc, false);
            }

            seen.insert(instruction_pointer);

            instruction_pointer += match self.0[instruction_pointer as usize] {
                Instruction::Nop(_) => 1,
                Instruction::Acc(val) => {
                    acc += val;
                    1
                }
                Instruction::Jmp(delta) => delta,
            }
        }
    }
}

fn part1() {
    let (acc, _) = Program::from_path("input.txt").execute();
    println!("{}", acc);
}

fn part2() {
    let base_program = Program::from_path("input.txt");

    let acc = base_program
        .iter()
        .enumerate()
        .find_map(|(i, inst)| match inst {
            Instruction::Nop(_) | Instruction::Jmp(_) => {
                let (acc, ok) = base_program.toggle_instruction(i).execute();

                if ok {
                    Some(acc)
                } else {
                    None
                }
            }
            _ => None,
        })
        .unwrap();

    println!("{}", acc);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}
