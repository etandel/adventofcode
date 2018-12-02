use std::env;
use std::collections::BTreeMap;
use std::fs;


fn count_chars(box_id: &str) -> BTreeMap<char, u32> {
    let mut count = BTreeMap::new();
    for c in box_id.chars() {
        *count.entry(c).or_insert(0) += 1;
    }
    return count;
}


fn has_exactly(n: u32, counts: &BTreeMap<char, u32>) -> bool {
    counts.values().find(|&&count| count == n).is_some()
}


fn part1(){
    let content = fs::read_to_string("input.txt").unwrap();
    let counts: Vec<BTreeMap<char, u32>> = content.lines().map(count_chars).collect();
    let twos = counts.iter()
                     .filter(|counts| has_exactly(2, &counts))
                     .count();
    let threes = counts.iter()
                       .filter(|counts| has_exactly(3, &counts))
                       .count();
    println!("{}", twos * threes);
}


fn hamming_distance(s1: &str, s2: &str) -> usize {
    s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2).count()
}


fn get_same_chars(s1: &str, s2: &str) -> String {
    s1.chars()
      .zip(s2.chars())
      .filter(|(c1, c2)| c1 == c2)
      .map(|(c1, _)| c1)
      .collect()
}


fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    for (i, s1) in lines.iter().enumerate() {
        for s2 in (&lines[i+1..]).iter() {
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
