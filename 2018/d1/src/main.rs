use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let result: i32 = content
        .lines()
        .map(|line| i32::from_str(line).unwrap())
        .sum();
    println!("{}", result);
}

fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let values: Vec<i32> = content
        .lines()
        .map(|line| i32::from_str(line).unwrap())
        .collect();

    let mut results: HashSet<i32> = HashSet::new();
    let mut result: i32 = 0;
    'outer: loop {
        for value in values.iter() {
            results.insert(result);
            result += value;
            if results.contains(&result) {
                break 'outer;
            }
        }
    }
    println!("{}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}
