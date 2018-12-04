extern crate regex;

use std::env;
use std::fs;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;


const CLOTH_SIZE: usize = 1000;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rect {
    id: u16,
    offset_x: u16,
    offset_y: u16,
    width: u16,
    height: u16,
}

impl Rect {
    fn intersects(&self, other: &Rect) -> bool {
        !(self.offset_x + self.width  < other.offset_x ||
          other.offset_x + other.width < self.offset_x ||
          self.offset_y + self.height < other.offset_y ||
          other.offset_y + other.height < self.offset_y)
    }
}

impl FromStr for Rect {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        Ok(Rect {
           id: captures[1].parse::<u16>()?,
           offset_x: captures[2].parse::<u16>()?,
           offset_y: captures[3].parse::<u16>()?,
           width: captures[4].parse::<u16>()?,
           height: captures[5].parse::<u16>()?,
        })
    }
}


fn pos(x: u16, y: u16) -> usize {
    CLOTH_SIZE * (y as usize) + (x as usize)
}


fn occupy_area(cloth: &mut Vec<u16>, rect: &Rect) {
    for y in 0..rect.height {
        for x in 0..rect.width {
            cloth[pos(rect.offset_x + x, rect.offset_y + y)] += 1;
        }
    }
}


fn part1() {
    let mut cloth: Vec<u16> = vec![0; CLOTH_SIZE * CLOTH_SIZE];
    let content = fs::read_to_string("input.txt").unwrap();
    for line in content.lines() {
        let rect = Rect::from_str(&line).unwrap();
        occupy_area(&mut cloth, &rect);
    }

    println!("{}", cloth.iter().filter(|&&x| x > 1).count());
}


fn part2() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut candidates: HashSet<Rect> = content.lines().map(Rect::from_str).map(Result::unwrap).collect();
    let candidates2: Vec<Rect> = candidates.iter().cloned().collect();
    for r1 in candidates2.iter() {
        for r2 in candidates2.iter() {
            if r1 != r2 && r1.intersects(r2) {
                candidates.remove(r1);
                candidates.remove(r2);
            }
        }
    }
    println!("{}", candidates.iter().next().unwrap().id);
}


fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}

