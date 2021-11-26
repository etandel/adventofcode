use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Map {
    rows: Vec<Vec<bool>>,
    ncols: usize,
    nrows: usize,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let rows: Vec<Vec<bool>> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let nrows = rows.len();
        let ncols = rows[0].len();

        Ok(Map { rows, ncols, nrows })
    }
}

impl Map {
    fn from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        fs::read_to_string(path)
            .map(|s| Map::from_str(&s))
            .unwrap()
            .unwrap()
    }

    fn has_tree(&self, row: usize, col: usize) -> bool {
        self.rows[row % self.nrows][col % self.ncols]
    }

    fn count_trees(&self, delta_row: usize, delta_col: usize) -> usize {
        let mut count = 0;
        let mut row = 0;
        let mut col = 0;

        while row < self.nrows {
            if self.has_tree(row, col) {
                count += 1;
            }
            row += delta_row;
            col += delta_col;
        }

        count
    }
}

fn part1() {
    let count = Map::from_file("input.txt").count_trees(1, 3);
    println!("{}", count);
}

fn part2() {
    let map = Map::from_file("input.txt");

    let deltas = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let mut total = 1;

    for (delta_row, delta_col) in deltas {
        let c = map.count_trees(delta_row, delta_col);
        total *= c
    }

    println!("{}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[1][..] {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Must pass either '1' or '2'."),
    }
}
