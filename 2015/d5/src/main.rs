use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn parse_text<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn is_nice_1<S: AsRef<str>>(s: S) -> bool {
    let chars = s.as_ref().chars().collect::<Vec<char>>();
    let nvowels = chars
        .iter()
        .filter(|&&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count();

    let has_double = chars
        .windows(2)
        .any(|pair| if let [x, y] = pair { x == y } else { false });

    let all_pairs_valid = chars.windows(2).all(|pair| {
        pair != ['a', 'b'] && pair != ['c', 'd'] && pair != ['p', 'q'] && pair != ['x', 'y']
    });

    nvowels >= 3 && has_double && all_pairs_valid
}

fn is_nice_2<S: AsRef<str>>(s: S) -> bool {
    let chars = s.as_ref().chars().collect::<Vec<char>>();

    let mut pair_positions = HashMap::with_capacity(chars.len());
    for (i, pair) in chars.windows(2).enumerate() {
        pair_positions
            .entry(pair)
            .or_insert_with(|| Vec::with_capacity(chars.len()))
            .push(i);
    }

    let pairs_ok = pair_positions.values().any(|positions| {
        positions.len() >= 2
            && positions
                .iter()
                .combinations(2)
                .filter(|pair| pair[0] + 1 != *pair[1])
                .count()
                >= 1
    });

    let repeat_ok = chars.windows(3).any(|window| {
        if let [x, _, y] = window {
            x == y
        } else {
            false
        }
    });

    repeat_ok && pairs_ok
}

fn part1() {
    let res = parse_text("input.txt")
        .lines()
        .filter(|s| is_nice_1(s))
        .count();
    println!("{}", res);
}

fn part2() {
    let res = parse_text("input.txt")
        .lines()
        .filter(|s| is_nice_2(s))
        .count();
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
    fn test_is_nice_1() {
        assert_eq!(is_nice_1("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_1("aaa"), true);

        assert_eq!(is_nice_1("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_1("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_2() {
        assert_eq!(is_nice_2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_2("xxyxx"), true);
        assert_eq!(is_nice_2("xyxaaaa"), true);

        assert_eq!(is_nice_2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_2("ieodomkazucvgmuy"), false);
        assert_eq!(is_nice_2("xyxaaa"), false);
    }
}
