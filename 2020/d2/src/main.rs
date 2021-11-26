use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Line {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if let [range_rule, letter_rule, password] =
            s.split_ascii_whitespace().collect::<Vec<_>>()[..]
        {
            if let [lower, upper] = range_rule
                .split("-")
                .filter_map(|p| usize::from_str(p).ok())
                .collect::<Vec<_>>()[..]
            {
                let letter = letter_rule.chars().next().unwrap();

                return Ok(Line {
                    lower,
                    upper,
                    letter,
                    password: password.to_string(),
                });
            }
        }

        Err(())
    }
}

fn count_valid_lines<F>(f: F) -> usize
where
    F: FnMut(&Line) -> bool,
{
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter_map(|l| Line::from_str(l).ok())
        .filter(f)
        .count()
}

fn is_valid_part1(line: &Line) -> bool {
    let count = line.password.chars().filter(|&c| c == line.letter).count();
    count >= line.lower && count <= line.upper
}

fn is_valid_part2(line: &Line) -> bool {
    let chars = line.password.chars().collect::<Vec<_>>();

    (chars[line.lower - 1] == line.letter) ^ (chars[line.upper - 1] == line.letter)
}

fn part1() {
    println!("{}", count_valid_lines(is_valid_part1));
}

fn part2() {
    println!("{}", count_valid_lines(is_valid_part2));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}
