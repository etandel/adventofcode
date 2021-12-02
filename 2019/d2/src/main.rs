use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type OpCode = usize;
type Program = Vec<OpCode>;

fn parse_program<P>(path: P) -> Program
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(OpCode::from_str)
        .filter_map(Result::ok)
        .collect()
}

const HALT: OpCode = 99;
const ADD: OpCode = 1;
const MUL: OpCode = 2;

fn run_program(mut prog: Program) -> Program {
    let mut pc = 0;

    loop {
        match prog[pc] {
            HALT => break,
            ADD => {
                let out_addr = prog[pc + 3];
                prog[out_addr] = prog[prog[pc + 1]] + prog[prog[pc + 2]];
            }
            MUL => {
                let out_addr = prog[pc + 3];
                prog[out_addr] = prog[prog[pc + 1]] * prog[prog[pc + 2]];
            }
            _ => panic!(
                "Invalid opcode {} at position {} in program {:#?}",
                prog[pc], pc, prog
            ),
        }

        pc += 4;
    }

    prog
}

fn part1() {
    let mut prog = parse_program("input.txt");
    prog[1] = 12;
    prog[2] = 2;
    let prog = run_program(prog);
    println!("{}", prog[0]);
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
    fn test_run_program() {
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);

        assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);

        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
