use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

type BingoNum = u64;

#[derive(Debug)]
struct Entry {
    val: BingoNum,
    marked: bool,
}

impl Entry {
    fn has_value(&self, n: BingoNum) -> bool {
        self.val == n
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Entry>>,
}

impl Board {
    const SIZE: usize = 5;

    fn from_lines(lines: &[&str]) -> Self {
        let rows = lines
            .iter()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| Entry {
                        marked: false,
                        val: n.parse().unwrap(),
                    })
                    .collect()
            })
            .collect();

        Self { rows }
    }

    fn mark(&mut self, num: BingoNum) {
        'outer: for row in self.rows.iter_mut() {
            for mut entry in row {
                if entry.has_value(num) {
                    entry.marked = true;
                    break 'outer;
                }
            }
        }
    }

    fn score(&self) -> BingoNum {
        self.rows
            .iter()
            .flat_map(|row| {
                row.iter().filter_map(|e| match e {
                    Entry { marked: false, val } => Some(val),
                    _ => None,
                })
            })
            .sum()
    }

    fn won(&self) -> bool {
        for row in self.rows.iter() {
            if row.iter().all(Entry::is_marked) {
                return true;
            }
        }

        for col in 0..Self::SIZE {
            if self.rows.iter().all(|row| row[col].is_marked()) {
                return true;
            }
        }

        false
    }
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn parse_bingo_sequence(input: &str) -> Vec<BingoNum> {
    input
        .lines()
        .next()
        .iter()
        .flat_map(|l| l.split(',').filter_map(|n| n.parse().ok()))
        .collect()
}

fn parse_boards(input: &str) -> Vec<Board> {
    let mut boards = Vec::new();

    let lines: Vec<&str> = input.lines().skip(1).collect();
    for i in (0..lines.len()).step_by(Board::SIZE + 1) {
        boards.push(Board::from_lines(&lines[i + 1..i + Board::SIZE + 1]))
    }

    boards
}

fn part1() {
    let input = read_input("input.txt");
    let bingo_seq = parse_bingo_sequence(&input);
    let mut boards = parse_boards(&input);

    for num in bingo_seq {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.won() {
                println!("{}", board.score() * num);
                return;
            }
        }
    }
}

fn part2() {
    let input = read_input("input.txt");
    let bingo_seq = parse_bingo_sequence(&input);
    let mut boards = parse_boards(&input);

    let mut won: HashMap<usize, BingoNum> = HashMap::new();

    let mut last: usize = 0;

    for num in bingo_seq {
        for (i, board) in boards.iter_mut().enumerate() {
            if !won.contains_key(&i) {
                board.mark(num);
            }

            if board.won() && !won.contains_key(&i) {
                won.insert(i, num);
                last = i;
            }
        }
    }

    let num = won.get(&last).unwrap();
    dbg!(&won, num);
    println!("{}", boards[last].score() * num);
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
