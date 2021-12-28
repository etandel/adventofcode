use std::collections::VecDeque;
use std::env;
use std::fs;
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    // depth, l / r
    Split(usize, Side),
    Explode(usize, Side),
}

type N = u32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Entry {
    Val(N),
    Open,
    Close,
}
use Entry::{Close, Open, Val};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Num {
    vals: VecDeque<Entry>,
}

impl Num {
    #[allow(dead_code)]
    fn p(l: Self, r: Self) -> Self {
        let mut vals = VecDeque::with_capacity(l.vals.len() + r.vals.len() + 2);
        vals.extend(l.vals.into_iter().chain(r.vals.into_iter()));
        vals.push_front(Open);
        vals.push_back(Close);
        Num { vals }
    }

    #[allow(dead_code)]
    fn pvl(l: N, r: Self) -> Self {
        Num {
            vals: VecDeque::from_iter([Open, Val(l)].into_iter().chain(r.vals).chain([Close])),
        }
    }


    #[allow(dead_code)]
    fn pvr(l: Self, r: N) -> Self {
        Num {
            vals: VecDeque::from_iter([Open].into_iter().chain(l.vals).chain([Val(r), Close])),
        }
    }

    #[allow(dead_code)]
    fn vp(l: N, r: N) -> Self {
        Num {
            vals: VecDeque::from_iter([Open, Val(l), Val(r), Close]),
        }
    }

    fn reduce(&self) -> Self {
       // TODO
       self.clone()
    }
}

impl FromStr for Num {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Num {
            vals: s
                .chars()
                .filter_map(|c| match c {
                    '[' => Some(Open),
                    ']' => Some(Close),
                    c => c.to_digit(10).map(|v| Val(v)),
                })
                .collect(),
        })
    }
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        Self::p(self, rhs)
    }
}

impl Sum<Num> for Option<Num> {
    fn sum<I>(mut i: I) -> Self
    where
        I: Iterator<Item = Num>,
    {
        match i.next() {
            Some(first) => Some(i.fold(first, |a, b| a + b)),
            None => None,
        }
    }
}

impl Num {
    fn mag(&self) -> N {
        todo!()
    }
}

fn read_input<P>(path: P) -> Vec<Num>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1() {
    let nums = read_input("input_example.txt");
    let res = nums.into_iter().sum::<Option<Num>>().unwrap().mag();
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
    use super::*;

    #[test]
    fn test_add() {
        fn assert_add(a: &str, b: &str, expected: &str) {
            assert_eq!(
                a.parse::<Num>().unwrap() + b.parse::<Num>().unwrap(),
                expected.parse::<Num>().unwrap()
            );
        }
        assert_add("[1, 2]", "[3, 4]", "[[1, 2], [3, 4]]")
    }

    #[test]
    fn test_from_str() {
        let s = "[1, 2]";
        let expected = Num::vp(1, 2);
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], 3]";
        let expected = Num::pvr(Num::vp(1, 2), 3);
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], [3, 4]]";
        let expected = Num::p(Num::vp(1, 2), Num::vp(3, 4));
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], [[3, 4], [5, [6, 7]]]]";
        let expected = Num::p(
            Num::vp(1, 2),
            Num::p(Num::vp(3, 4), Num::pvl(5, Num::vp(6, 7))),
        );
        assert_eq!(s.parse(), Ok(expected));
    }

    #[test]
    fn test_mag() {
        let p = Num::p(Num::vp(1, 2), Num::pvr(Num::vp(3, 4), 5));
        assert_eq!(p.mag(), 143);

        let p = Num::p(
            Num::p(
                Num::pvr(Num::vp(0, 7), 4),
                Num::p(Num::vp(7, 8), Num::vp(6, 0)),
            ),
            Num::vp(8, 1),
        );
        assert_eq!(p.mag(), 1384);
    }
}
