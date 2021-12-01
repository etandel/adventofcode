use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Dim = usize;

enum Dir {
    Forward(Dim),
    Up(Dim),
    Down(Dim),
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(match parts.as_slice() {
            ["forward", x] => Dir::Forward(x.parse().unwrap()),
            ["up", x] => Dir::Up(x.parse().unwrap()),
            ["down", x] => Dir::Down(x.parse().unwrap()),
            _ => panic!("Invalid instruction {}", s),
        })
    }
}

fn parse_instructions<P>(path: P) -> Vec<Dir>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(Dir::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn depth_folder((hor, dep): (Dim, Dim), inst: &Dir) -> (Dim, Dim) {
    match inst {
        Dir::Up(x) => (hor, dep.saturating_sub(*x)),
        Dir::Down(x) => (hor, dep + x),
        Dir::Forward(x) => (hor + x, dep),
    }
}

fn aim_folder((hor, dep, aim): (Dim, Dim, Dim), inst: &Dir) -> (Dim, Dim, Dim) {
    match inst {
        Dir::Up(x) => (hor, dep, aim.saturating_sub(*x)),
        Dir::Down(x) => (hor, dep, aim + x),
        Dir::Forward(x) => (hor + x, dep + aim * x, aim),
    }
}

fn part1() {
    let instructions = parse_instructions("input.txt");
    let (hor, dep): (Dim, Dim) = instructions.iter().fold((0, 0), depth_folder);

    println!("{}", hor * dep);
}

fn part2() {
    let instructions = parse_instructions("input.txt");
    let (hor, dep, _): (Dim, Dim, Dim) = instructions.iter().fold((0, 0, 0), aim_folder);

    println!("{}", hor * dep);
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
    #[test]
    fn test_() {}
}
