use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

type Point = [u32; 2];

#[derive(Debug, Clone, Copy)]
enum Fold {
    Hor(u32),
    Ver(u32),
}

fn parse_instruction(line: &str) -> Fold {
    let mut s = line.split_ascii_whitespace().last().unwrap().split('=');

    match (s.next(), s.next()) {
        (Some("y"), Some(t)) => Fold::Hor(t.parse().unwrap()),
        (Some("x"), Some(t)) => Fold::Ver(t.parse().unwrap()),
        _ => panic!("Invalid instruction: {}", line),
    }
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn part1() {
    let input = read_input("input.txt");
    let mut lines = input.lines();

    let mut points: HashSet<Point> = HashSet::new();

    for line in lines.by_ref() {
        if line == "" {
            break;
        }

        let mut s = line.split(',');
        points.insert([
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        ]);
    }

    for line in lines {
        let mut newpoints = HashSet::with_capacity(points.len());
        match parse_instruction(line) {
            f @ Fold::Ver(row) => {
                for &[x, y] in &points {
                    newpoints.insert(if x > row {
                        [row - (x - row), y]
                    } else if x < row {
                        [x, y]
                    } else {
                        panic!("Cannot fold point {:?} that sits on fold {:?}", (x, y), f);
                    });
                }
            }

            f @ Fold::Hor(col) => {
                for &[x, y] in &points {
                    newpoints.insert(if y > col {
                        [x, col - (y - col)]
                    } else if y < col {
                        [x, y]
                    } else {
                        panic!("Cannot fold point {:?} that sits on fold {:?}", (x, y), f);
                    });
                }
            }
        }

        points = newpoints;
        break;
    }

    println!("{}", points.len());
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
