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
struct Bulb(usize);

impl Bulb {
    fn is_lit(&self) -> bool {
        self.0 > 0
    }

    fn exec_1(&self, op: &Op) -> Self {
        match op {
            Op::TurnOn => Self(1),
            Op::TurnOff => Self(0),
            Op::Toggle => Self(self.0 ^ 1),
        }
    }

    fn exec_2(&self, op: &Op) -> Self {
        match op {
            Op::TurnOn => Self(self.0 + 1),
            Op::TurnOff => Self(self.0.saturating_sub(1)),
            Op::Toggle => Self(self.0 + 2),
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
    grid: Vec<Bulb>,
}

impl Grid {
    const LEN: usize = 1000;

    fn new() -> Self {
        Self {
            grid: vec![Bulb(0); Self::LEN * Self::LEN],
        }
    }

    fn points(&self, ((left, top), (right, bot)): Rect) -> impl Iterator<Item = Point> {
        (left..=right).flat_map(move |col| (top..=bot).map(move |row| (col, row)))
    }

    fn get_index(&self, (col, row): Point) -> usize {
        row * Self::LEN + col
    }

    fn get(&self, point: Point) -> &Bulb {
        &self.grid[self.get_index(point)]
    }

    fn execute_instruction<F>(&mut self, Instruction { op, rect }: Instruction, executor: F)
    where
        F: Fn(&Bulb, &Op) -> Bulb,
    {
        for point in self.points(rect) {
            let i = self.get_index(point);
            self.grid[i] = executor(&self.grid[i], &op);
        }
    }

    fn from_instructions<I, F>(instructions: I, executor: &F) -> Self
    where
        I: IntoIterator<Item = Instruction>,
        F: Fn(&Bulb, &Op) -> Bulb,
    {
        let mut grid = Self::new();
        for inst in instructions {
            grid.execute_instruction(inst, executor);
        }
        grid
    }

    fn count_lit(&self) -> usize {
        self.points(((0, 0), (Grid::LEN - 1, Grid::LEN - 1)))
            .map(|p| self.get(p))
            .copied()
            .filter(Bulb::is_lit)
            .count()
    }

    fn total_brightness(&self) -> usize {
        self.points(((0, 0), (Grid::LEN - 1, Grid::LEN - 1)))
            .map(|p| self.get(p).0)
            .sum::<usize>()
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
    let res = Grid::from_instructions(instructions, &Bulb::exec_1).count_lit();

    println!("{}", res);
}

fn part2() {
    let instructions = parse_instructions("input.txt");
    let res = Grid::from_instructions(instructions, &Bulb::exec_2).total_brightness();

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
    fn test_() {}
}
