use std::env;
use std::fs;
use std::str::FromStr;


fn part1() {
    let mut result: Option<u64> = None;

    let mut expenses: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| u64::from_str(line).unwrap())
        .collect();

    expenses.sort();

    'outer: for (i, val1) in expenses.iter().enumerate() {
        for val2 in expenses.iter().skip(i) {
            if val1 + val2 == 2020 {
                result = Some(val1 * val2);
                break 'outer;

            } else if val1 + val2 > 2020 {
                break
            }
        }
    }

    println!("{}", result.unwrap());
}


fn part2() {
    let mut result: Option<u64> = None;

    let mut expenses: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| u64::from_str(line).unwrap())
        .collect();

    expenses.sort();

    'outer: for (i, val1) in expenses.iter().enumerate() {
        for val2 in expenses.iter().skip(i) {
            for val3 in expenses.iter().skip(i) {
                if val1 + val2 + val3 == 2020 {
                    result = Some(val1 * val2 * val3);
                    break 'outer;

                } else if val1 + val2 + val3 > 2020 {
                    break
                }
            }
        }
    }

    println!("{}", result.unwrap());
}



fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}

