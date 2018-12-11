extern crate regex;

use std::collections::{BTreeMap, BTreeSet};
use std::env;
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
        Ok(Star{
            x: captures[1].parse::<i64>()?,
            y: captures[2].parse::<i64>()?,
            vx: captures[3].parse::<i64>()?,
            vy: captures[4].parse::<i64>()?,
        })
    }
}


fn read_points<'a>(raw_points: &'a str) -> impl Iterator<Item = Result<Star, ParseIntError>> + 'a {
    raw_points.lines().map(Star::from_str)
}


fn tick_forwards(state: &mut Vec<Star>) {
    for i in 0..state.len() {
        state[i].tick_forwards();
    }
}


fn tick_backwards(state: &mut Vec<Star>) {
    for i in 0..state.len() {
        state[i].tick_backwards();
    }
}

fn centroid(state: &Vec<Star>) -> (i64, i64){
    let x = state.iter().map(|s| s.x).sum::<i64>() / state.len() as i64;
    let y = state.iter().map(|s| s.y).sum::<i64>() / state.len() as i64;
    (x, y)
}


fn avg_distance_to(state: &Vec<Star>, x: i64, y: i64) -> i64 {
    state.iter().map(|s| (s.x - x).abs() + (s.y - y).abs()).sum()
}


fn sim(input: &str) -> (usize, String) {
    let content = fs::read_to_string(input).unwrap();
    let mut state: Vec<Star> = read_points(&content).collect::<Result<Vec<_>, _>>().unwrap();

    let mut n_steps: usize = 0;
    let (cx, cy) = centroid(&state);
    let mut min_distance = avg_distance_to(&state, cx, cy);
    loop {
        tick_forwards(&mut state);
        n_steps += 1;

        let (cx, cy) = centroid(&state);
        let new_distance = avg_distance_to(&state, cx, cy);

        if new_distance <= min_distance {
            min_distance = new_distance;
        } else {
            tick_backwards(&mut state);
            n_steps -= 1;
            break;
        }

    }

    let points: BTreeSet<(i64, i64)> = state.iter().map(|s| (s.x, s.y)).collect();


    let min_x = state.iter().map(|s| s.x).min().unwrap();
    let min_y = state.iter().map(|s| s.y).min().unwrap();
    let max_x = state.iter().map(|s| s.x).max().unwrap();
    let max_y = state.iter().map(|s| s.y).max().unwrap();

    let mut output: Vec<u8> = Vec::with_capacity((max_x - min_x + 1) as usize
                                                 * (max_y - min_y + 1) as usize
                                                 + (max_y - min_y) as usize
                                                 + 1);
    for y in min_y .. max_y + 1 {
        for x in min_x .. max_x + 1 { 
            if points.contains(&(x, y)) {
                output.push(b'#');
            } else {
                output.push(b'.');
            }
        }
        output.push(b'\n');
    }

    (n_steps, String::from_utf8(output).unwrap())
}


fn part1(input: &str) {
    let (_, output) = sim(input);
    println!("{}", output);
}


fn part2(input: &str) {
    let (count, _) = sim(input);
    println!("{}", count);
}


fn main() {
    let args: Vec<_> = env::args().collect();
    match args[1].as_str() {
        "1" => part1(args[2].as_str()),
        _ => part2(args[2].as_str()),
    };
}

