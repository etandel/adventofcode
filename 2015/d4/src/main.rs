use std::env;
use std::fs;
use std::path::Path;

use md5;

fn parse_key<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn find_prefix_5(key: &str) -> u64 {
    (1..u64::MAX)
        .find_map(|i| {
            let digest = md5::compute(format!("{}{}", key, i));
            if let [0, 0, x] = &digest[0..3] {
                if x >> 4 == 0 {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap()
}

fn find_prefix_6(key: &str) -> u64 {
    (1..u64::MAX)
        .find_map(|i| {
            let digest = md5::compute(format!("{}{}", key, i));
            if let [0, 0, 0] = &digest[0..3] {
                Some(i)
            } else {
                None
            }
        })
        .unwrap()
}

fn part1() {
    let res = find_prefix_5(&parse_key("input.txt"));
    println!("{}", res);
}

fn part2() {
    let res = find_prefix_6(&parse_key("input.txt"));
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
mod tests {}
