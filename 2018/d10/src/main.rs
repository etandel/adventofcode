extern crate regex;

use std::collections::BTreeSet;
use std::env;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

const POINT_REGEX: &str = r"^position=<\s*(.+),\s*(.+)> velocity=<\s*(.+),\s*(.+)>$";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Star {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Star {
    fn tick_forwards(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn tick_backwards(&mut self) {
        self.x -= self.vx;
        self.y -= self.vy;
    }
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(POINT_REGEX).unwrap();
        let captures = re.captures(s).unwrap();
        Ok(Star {
            x: captures[1].parse::<i64>()?,
            y: captures[2].parse::<i64>()?,
            vx: captures[3].parse::<i64>()?,
            vy: captures[4].parse::<i64>()?,
        })
    }
}

struct State {
    time: usize,
    state: Vec<Star>,
}

impl State {
    fn tick_forwards(&mut self) {
        for star in self.state.iter_mut() {
            star.tick_forwards();
        }
        self.time += 1;
    }

    fn tick_backwards(&mut self) {
        for star in self.state.iter_mut() {
            star.tick_backwards();
        }
        self.time -= 1;
    }

    fn centroid(&self) -> (i64, i64) {
        let x = self.state.iter().map(|s| s.x).sum::<i64>() / self.state.len() as i64;
        let y = self.state.iter().map(|s| s.y).sum::<i64>() / self.state.len() as i64;
        (x, y)
    }

    fn avg_distance_to(&self, x: i64, y: i64) -> i64 {
        self.state
            .iter()
            .map(|s| (s.x - x).abs() + (s.y - y).abs())
            .sum()
    }
}

impl FromStr for State {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<State, Self::Err> {
        let state = s
            .lines()
            .map(Star::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(State {
            state,
            time: 0,
        })
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let points: BTreeSet<(i64, i64)> = self.state.iter().map(|s| (s.x, s.y)).collect();

        let min_x = self.state.iter().map(|s| s.x).min().unwrap();
        let min_y = self.state.iter().map(|s| s.y).min().unwrap();
        let max_x = self.state.iter().map(|s| s.x).max().unwrap();
        let max_y = self.state.iter().map(|s| s.y).max().unwrap();

        let mut output: Vec<u8> = Vec::with_capacity(
            (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize
                + (max_y - min_y) as usize
                + 1,
        );
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if points.contains(&(x, y)) {
                    output.push(b'#');
                } else {
                    output.push(b'.');
                }
            }
            output.push(b'\n');
        }

        write!(f, "{}", String::from_utf8(output).unwrap())
    }
}

fn sim(input: &str) -> State {
    let content = fs::read_to_string(input).unwrap();
    let mut state = State::from_str(&content).unwrap();

    let (cx, cy) = state.centroid();
    let mut min_distance = state.avg_distance_to(cx, cy);
    loop {
        state.tick_forwards();

        let (cx, cy) = state.centroid();
        let new_distance = state.avg_distance_to(cx, cy);

        if new_distance <= min_distance {
            min_distance = new_distance;
        } else {
            state.tick_backwards();
            break;
        }
    }

    state
}

fn part1(input: &str) {
    let state = sim(input);
    println!("{}", state);
}

fn part2(input: &str) {
    let state = sim(input);
    println!("{}", state.time);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(args[2].as_str()),
        _ => part2(args[2].as_str()),
    };
}
