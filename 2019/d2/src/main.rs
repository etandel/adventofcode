use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type OpCode = usize;
type OpArray = Vec<OpCode>;

struct IntCodeComputer {
    program: OpArray,
    memory: OpArray,
}

impl IntCodeComputer {
    const HALT: OpCode = 99;
    const ADD: OpCode = 1;
    const MUL: OpCode = 2;

    fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_source(&fs::read_to_string(path).unwrap())
    }

    fn from_source(source: &str) -> Self {
        let prog = source
            .split(',')
            .map(OpCode::from_str)
            .filter_map(Result::ok);

        Self::from_program(prog)
    }

    fn from_program<I>(prog: I) -> Self
    where
        I: IntoIterator<Item = OpCode>,
    {
        let p: OpArray = prog.into_iter().collect();
        Self {
            program: p.clone(),
            memory: p,
        }
    }

    fn set(&mut self, addr: OpCode, value: OpCode) -> OpCode {
        let old = self.memory[addr];
        self.memory[addr] = value;
        old
    }

    fn get(&self, addr: OpCode) -> OpCode {
        self.memory[addr]
    }

    fn run_until_halt(&mut self) {
        let mut pc = 0;

        loop {
            match self.memory[pc] {
                Self::HALT => break,
                Self::ADD => {
                    let out_addr = self.memory[pc + 3];
                    self.memory[out_addr] =
                        self.memory[self.memory[pc + 1]] + self.memory[self.memory[pc + 2]];
                }
                Self::MUL => {
                    let out_addr = self.memory[pc + 3];
                    self.memory[out_addr] =
                        self.memory[self.memory[pc + 1]] * self.memory[self.memory[pc + 2]];
                }
                _ => panic!(
                    "Invalid opcode {} at position {} in memory {:#?}",
                    self.memory[pc], pc, self.memory
                ),
            }

            pc += 4;
        }
    }

    fn reset(&mut self) {
        self.memory = self.program.clone();
    }
}

fn part1() {
    let mut cmp = IntCodeComputer::from_path("input.txt");

    cmp.set(1, 12);
    cmp.set(2, 2);
    cmp.run_until_halt();

    println!("{}", cmp.get(0));
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
    use super::*;

    #[test]
    fn test_run_until_halt() {
        let mut cmp = IntCodeComputer::from_program(vec![1, 0, 0, 0, 99]);
        cmp.run_until_halt();
        assert_eq!(cmp.memory, vec![2, 0, 0, 0, 99]);

        let mut cmp = IntCodeComputer::from_program(vec![2, 3, 0, 3, 99]);
        cmp.run_until_halt();
        assert_eq!(cmp.memory, vec![2, 3, 0, 6, 99]);

        let mut cmp = IntCodeComputer::from_program(vec![2, 4, 4, 5, 99, 0]);
        cmp.run_until_halt();
        assert_eq!(cmp.memory, vec![2, 4, 4, 5, 99, 9801]);

        let mut cmp = IntCodeComputer::from_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        cmp.run_until_halt();
        assert_eq!(cmp.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
