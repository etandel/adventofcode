use std::collections::BTreeMap;
use std::env;
use std::fs;

fn count_chars(box_id: &str) -> BTreeMap<char, u32> {
    let mut count = BTreeMap::new();
    for c in box_id.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}

fn has_exactly(n: u32, count: &BTreeMap<char, u32>) -> bool {
    count.values().find(|&&c| c == n).is_some()
}

fn count_exactly(n: u32, counts: &Vec<BTreeMap<char, u32>>) -> usize {
    counts.iter().filter(|count| has_exactly(n, &count)).count()
}

fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let counts: Vec<BTreeMap<char, u32>> = content.lines().map(count_chars).collect();
    println!("{}", count_exactly(2, &counts) * count_exactly(3, &counts));
}

fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn get_same_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect()
}

fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (i, s1) in lines.iter().enumerate() {
        for s2 in (&lines[i + 1..]).iter() {
            if hamming_distance(s1, s2) == 1 {
                println!("{}", get_same_chars(s1, s2));
                return;
            }
        }
    }
}

fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}
