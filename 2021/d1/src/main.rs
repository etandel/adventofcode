use std::env;
use std::fs;
use std::path::Path;

type Depth = u64;

fn parse_depths<P>(path: P) -> Vec<Depth>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::parse::<Depth>)
        .filter_map(Result::ok)
        .collect()
}

fn count_increased(depths: &Vec<Depth>) -> usize {
    depths
        .windows(2)
        .map(|pairs| match pairs {
            [x, y] if y > x => 1,
            _ => 0,
        })
        .sum()
}

fn part1() {
    let depths = parse_depths("input.txt");
    let res = count_increased(&depths);
    println!("{}", res);
}

fn part2() {
    let depths = parse_depths("input.txt");
    let window_sums: Vec<Depth> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    let res = count_increased(&window_sums);
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
    #[test]
    fn test_adjacent_seats() {}
}
