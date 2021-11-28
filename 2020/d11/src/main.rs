use std::env;
use std::fs;
use std::path::Path;

use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pos {
    Floor,
    Empty,
    Occupied,
}

impl Pos {
    fn is_empty(&self) -> bool {
        *self == Pos::Empty
    }

    fn is_occupied(&self) -> bool {
        *self == Pos::Occupied
    }

    fn toggle(&self) -> Self {
        match self {
            Self::Empty => Self::Occupied,
            Self::Occupied => Self::Empty,
            x => *x,
        }
    }
}

type Row = Vec<Pos>;
type Layout = Vec<Row>;

fn parse_layout_row(s: &str) -> Row {
    s.chars()
        .map(|c| match c {
            '.' => Pos::Floor,
            'L' => Pos::Empty,
            '#' => Pos::Occupied,
            _ => panic!("Invalid position: {}", c),
        })
        .collect()
}

fn parse_layout<P>(path: P) -> Layout
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_layout_row)
        .collect()
}

fn add_delta(x: usize, delta: i64) -> Option<usize> {
    //((x as i64) + delta).try_into().ok()
    if delta < 0 {
        x.checked_sub(delta.abs() as usize)
    } else {
        Some(x + (delta.abs() as usize))
    }
}

lazy_static! {
    static ref DELTAS: Vec<(i64, i64)> = [
        (0 + 0, 0 + 1),
        (0 - 1, 0 + 1),
        (0 - 1, 0 + 0),
        (0 - 1, 0 - 1),
        (0 + 0, 0 - 1),
        (0 + 1, 0 - 1),
        (0 + 1, 0 + 0),
        (0 + 1, 0 + 1),
    ]
    .iter()
    .filter(|(di, dj)| *di != 0 || *dj != 0)
    .copied()
    .collect();
}

fn adjacent_seats(layout: &Layout, i: usize, j: usize) -> Vec<Pos> {
    DELTAS
        .iter()
        .filter_map(|(di, dj)| {
            let checked = (add_delta(i, *di), add_delta(j, *dj));
            if let (Some(newi), Some(newj)) = checked {
                Some((newi, newj))
            } else {
                None
            }
        })
        .filter_map(|(i, j)| layout.get(i).and_then(|row| row.get(j)))
        .filter(|p| **p != Pos::Floor)
        .copied()
        .collect()
}

fn should_toggle(layout: &Layout, i: usize, j: usize) -> bool {
    match layout[i][j] {
        Pos::Empty => adjacent_seats(layout, i, j).iter().all(Pos::is_empty),
        Pos::Occupied => {
            adjacent_seats(layout, i, j)
                .iter()
                .copied()
                .filter(Pos::is_occupied)
                .count()
                >= 4
        }
        _ => false,
    }
}

fn print_layout(layout: &Layout) {
    let mut s = String::with_capacity(layout.len() * layout[0].len());

    for row in layout {
        for position in row {
            let c = match position {
                Pos::Floor => '.',
                Pos::Empty => 'L',
                Pos::Occupied => '#',
            };
            s.push(c);
        }
        s.push('\n')
    }
    println!("{}", s);
}

fn iterate_layout(layout: &Layout) -> (Layout, bool) {
    let mut new_layout = layout.clone();

    let mut changed = false;

    for i in 0..layout.len() {
        for j in 0..layout[0].len() {
            if should_toggle(layout, i, j) {
                new_layout[i][j] = layout[i][j].toggle();
                changed = true;
            }
        }
    }

    (new_layout, changed)
}

fn count_occupied(layout: &Layout) -> usize {
    layout
        .iter()
        .flatten()
        .filter(|p| **p == Pos::Occupied)
        .count()
}

fn part1() {
    let mut layout = parse_layout("input.txt");
    while {
        let (new_layout, changed) = iterate_layout(&layout);
        layout = new_layout;
        changed
    } {}
    println!("{}", count_occupied(&layout));
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
    use super::*;

    #[test]
    fn test_adjacent_seats() {
        let layout = vec![vec![Pos::Floor]];
        let got = adjacent_seats(&layout, 0, 0);
        assert!(got == vec![]);

        let layout = vec![vec![Pos::Empty]];
        let got = adjacent_seats(&layout, 0, 0);
        assert!(got == vec![]);

        let layout = vec![vec![Pos::Empty, Pos::Empty]];
        let got = adjacent_seats(&layout, 0, 0);
        assert!(got == vec![Pos::Empty]);

        let layout = vec![
            vec![Pos::Empty, Pos::Empty],
            vec![Pos::Occupied, Pos::Floor],
        ];
        let got = adjacent_seats(&layout, 0, 0);
        assert!(got == vec![Pos::Empty, Pos::Occupied]);

        let layout = vec![
            vec![Pos::Floor, Pos::Floor, Pos::Floor],
            vec![Pos::Floor, Pos::Empty, Pos::Floor],
            vec![Pos::Floor, Pos::Floor, Pos::Floor],
        ];
        let got = adjacent_seats(&layout, 1, 1);
        assert!(got == vec![]);
    }
}
