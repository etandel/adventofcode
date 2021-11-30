use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

type Dim = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Box {
    l: Dim,
    w: Dim,
    h: Dim,
}

impl FromStr for Box {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if let [l, w, h] = s
            .split("x")
            .map(Dim::from_str)
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Box {
                l: *l,
                w: *w,
                h: *h,
            })
        } else {
            Err(())
        }
    }
}

impl Box {
    fn smallest_side_dims(&self) -> (Dim, Dim) {
        let mut dims = [self.l, self.w, self.h];
        dims.sort();
        (dims[0], dims[1])
    }

    fn volume(&self) -> Dim {
        self.l * self.w * self.h
    }

    fn wrapping_paper_area(&self) -> Dim {
        let surface_area = 2 * (self.l * self.w + self.w * self.h + self.l * self.h);
        let slack = {
            let (x, y) = self.smallest_side_dims();
            x * y
        };
        surface_area + slack
    }

    fn ribbon_length(&self) -> Dim {
        let (x, y) = self.smallest_side_dims();
        2 * (x + y) + self.volume()
    }
}

fn parse_boxes<P>(path: P) -> Vec<Box>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(Box::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn part1() {
    let res: Dim = parse_boxes("input.txt")
        .iter()
        .map(Box::wrapping_paper_area)
        .sum();

    println!("{}", res);
}

fn part2() {
    let res: Dim = parse_boxes("input.txt")
        .iter()
        .map(Box::ribbon_length)
        .sum();
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
mod tests {}
