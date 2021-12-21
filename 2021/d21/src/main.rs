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

fn play(mut p1_pos: u64, mut p2_pos: u64) -> u64 {
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut rolls = (1..=100).cycle();
    let mut nrolls = 0;

    loop {
        dbg!(p1_pos, p1_score, p2_pos, p2_score);

        nrolls += 3;
        p1_pos = ((p1_pos + rolls.next().unwrap() + rolls.next().unwrap() + rolls.next().unwrap())
            - 1)
            % 10
            + 1;
        p1_score += p1_pos;
        if p1_score >= 1000 {
            return nrolls * p2_score;
        }

        nrolls += 3;
        p2_pos = ((p2_pos + rolls.next().unwrap() + rolls.next().unwrap() + rolls.next().unwrap())
            - 1)
            % 10
            + 1;
        p2_score += p2_pos;
        if p2_score >= 1000 {
            return nrolls * p1_score;
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
