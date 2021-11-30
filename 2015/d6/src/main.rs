use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s {
            "on" => Self::TurnOn,
            "off" => Self::TurnOff,
            "toggle" => Self::Toggle,
            _ => panic!("Invalid op: {}", s),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    On,
    Off,
}

impl State {
    fn exec(&self, op: &Op) -> State {
        match op {
            Op::TurnOn => State::On,
            Op::TurnOff => State::Off,
            Op::Toggle => self.toggle(),
        }
    }

    fn toggle(&self) -> State {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

type Point = (usize, usize);
type Rect = (Point, Point);

fn parse_point(s: &str) -> Point {
    if let [x, y] = s.split(',').collect::<Vec<_>>().as_slice() {
        (usize::from_str(x).unwrap(), usize::from_str(y).unwrap())
    } else {
        panic!("Invalid point: {}", s);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    op: Op,
    rect: Rect,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let splitted: Vec<&str> = s.split_ascii_whitespace().collect();

        match splitted.as_slice() {
            [_toggle, top_left, _through, bot_right] => Ok(Self {
                op: Op::Toggle,
                rect: (parse_point(top_left), parse_point(bot_right)),
            }),
            [_turn, op, top_left, _through, bot_right] => Ok(Self {
                op: Op::from_str(op).unwrap(),
                rect: (parse_point(top_left), parse_point(bot_right)),
            }),
            _ => Err(()),
        }
    }
}

struct Grid {
    grid: Vec<State>,
}

impl Grid {
    const LEN: usize = 1000;

    fn new() -> Self {
        Self {
            grid: vec![State::Off; Self::LEN * Self::LEN],
        }
    }

    fn points(&self, ((left, top), (right, bot)): Rect) -> impl Iterator<Item = Point> {
        (left..=right).flat_map(move |col| (top..=bot).map(move |row| (col, row)))
    }

    fn get_index(&self, (col, row): Point) -> usize {
        row * Self::LEN + col
    }

    fn get(&self, point: Point) -> &State {
        &self.grid[self.get_index(point)]
    }

    fn update_point(&mut self, point: Point, op: &Op) {
        let i = self.get_index(point);
        self.grid[i] = self.grid[i].exec(op);
    }

    fn execute_instruction(&mut self, Instruction { op, rect }: Instruction) {
        for point in self.points(rect) {
            self.update_point(point, &op);
        }
    }
}

fn parse_instructions<P>(path: P) -> Vec<Instruction>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn part1() {
    let instructions = parse_instructions("input.txt");
    let mut grid = Grid::new();

    for inst in instructions {
        grid.execute_instruction(inst);
    }

    let res = grid
        .points(((0, 0), (Grid::LEN - 1, Grid::LEN - 1)))
        .map(|p| grid.get(p))
        .filter(|s| **s == State::On)
        .count();

    println!("{}", res);
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
