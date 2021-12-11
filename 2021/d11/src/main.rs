use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Add;
use std::path::Path;

type Octo = u32;

const N: i32 = 10;

type Grid = Vec<Vec<Octo>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Pos {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        let new = self + rhs;
        if new.0 < 0 || new.0 >= N || new.1 < 0 || new.1 >= N {
            None
        } else {
            Some(new)
        }
    }
}

fn neighbors(pos: Pos) -> Vec<Pos> {
    let deltas: [Pos; 8] = [
        Pos(-1, -1),
        Pos(-1, 0),
        Pos(-1, 1),
        Pos(0, -1),
        Pos(0, 1),
        Pos(1, -1),
        Pos(1, 0),
        Pos(1, 1),
    ];

    deltas.iter().filter_map(|d| pos.checked_add(*d)).collect()
}

fn read_grid<P>(path: P) -> Grid
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1() {
    let mut grid = read_grid("input.txt");

    let mut total = 0;

    for _ in 0..100 {
        let mut to_flash: Vec<Pos> = Vec::with_capacity((N * N) as usize);
        let mut flashed: HashSet<Pos> = HashSet::with_capacity((N * N) as usize);

        for y in 0..(N as usize) {
            let row = &mut grid[y];

            for x in 0..(N as usize) {
                row[x] += 1;
                if row[x] > 9 {
                    let p = Pos(y as i32, x as i32);
                    to_flash.push(p);
                    flashed.insert(p);
                }
            }
        }

        while let Some(next) = to_flash.pop() {
            for neighbor in neighbors(next) {
                grid[neighbor.0 as usize][neighbor.1 as usize] += 1;

                if grid[neighbor.0 as usize][neighbor.1 as usize] > 9
                    && !flashed.contains(&neighbor)
                {
                    to_flash.push(neighbor);
                    flashed.insert(neighbor);
                }
            }
        }

        for pos in &flashed {
            grid[pos.0 as usize][pos.1 as usize] = 0;
        }

        total += flashed.len();
    }

    println!("{}", total);
}

fn part2() {
    let mut grid = read_grid("input.txt");

    let mut step = 0;
    loop {
        let mut to_flash: Vec<Pos> = Vec::with_capacity((N * N) as usize);
        let mut flashed: HashSet<Pos> = HashSet::with_capacity((N * N) as usize);

        for y in 0..(N as usize) {
            let row = &mut grid[y];

            for x in 0..(N as usize) {
                row[x] += 1;
                if row[x] > 9 {
                    let p = Pos(y as i32, x as i32);
                    to_flash.push(p);
                    flashed.insert(p);
                }
            }
        }

        while let Some(next) = to_flash.pop() {
            for neighbor in neighbors(next) {
                grid[neighbor.0 as usize][neighbor.1 as usize] += 1;

                if grid[neighbor.0 as usize][neighbor.1 as usize] > 9
                    && !flashed.contains(&neighbor)
                {
                    to_flash.push(neighbor);
                    flashed.insert(neighbor);
                }
            }
        }

        for pos in &flashed {
            grid[pos.0 as usize][pos.1 as usize] = 0;
        }

        step += 1;

        if flashed.len() == (N * N) as usize {
            break;
        }
    }

    println!("{}", step);
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
