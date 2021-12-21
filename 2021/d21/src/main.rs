use std::env;
use std::fs;
use std::path::Path;

fn read_starting_pos<P>(path: P) -> [u64; 2]
where
    P: AsRef<Path>,
{
    let poss = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().last().unwrap().to_digit(10).unwrap())
        .collect::<Vec<_>>();

    [poss[0] as u64, poss[1] as u64]
}

fn play(p1_pos: u64, p2_pos: u64) -> u64 {
    let mut score = [0, 0];
    let mut pos = [p1_pos, p2_pos];

    let mut nrolls = 0;
    let mut rolls = (1..=100).cycle();

    loop {
        for player in 0..=1 {
            nrolls += 3;
            pos[player] = ((pos[player] + rolls.by_ref().take(3).sum::<u64>()) - 1) % 10 + 1;

            score[player] += pos[player];

            if score[player] >= 1000 {
                return nrolls * score[(player + 1) % 2];
            }
        }
    }
}

fn part1() {
    let [p1_pos, p2_pos] = read_starting_pos("input.txt");
    println!("{}", play(p1_pos, p2_pos));
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
    #[test]
    fn test_() {}
}
