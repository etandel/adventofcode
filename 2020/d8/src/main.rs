use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
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

type Program = Vec<Instruction>;

fn parse_program<P>(path: P) -> Program
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| Instruction::from_str(l).unwrap())
        .collect()
}

fn part1() {
    let program = parse_program("input.txt");

    let mut seen: HashSet<usize> = HashSet::with_capacity(program.len());

    let mut acc = 0;
    let mut instruction_pointer = 0;

    while !seen.contains(&instruction_pointer) {
        seen.insert(instruction_pointer);

        match program[instruction_pointer] {
            Instruction::Nop(_) => instruction_pointer += 1,
            Instruction::Acc(val) => {
                acc += val;
                instruction_pointer += 1;
            }
            Instruction::Jmp(delta) => {
                //dbg!(program.len(), instruction_pointer, delta);
                if delta > 0 {
                    instruction_pointer += delta.abs() as usize;
                } else {
                    instruction_pointer -= delta.abs() as usize;
                }
            }
        }
    }

    println!("{}", acc);
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
