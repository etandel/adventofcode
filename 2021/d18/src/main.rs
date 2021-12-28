use std::env;
use std::fs;
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Side {
    L, R
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    // depth, l / r
    Split(usize, Side),
    Explode(usize, Side),
}

type W<T> = Rc<T>;
type N = u32;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Num {
    Val(N),
    Pair(W<Num>, W<Num>),
}

impl Num {
    #[allow(dead_code)]
    fn p(l: Self, r: Self) -> Self {
        Self::Pair(W::new(l), W::new(r))
    }

    #[allow(dead_code)]
    fn vp(l: N, r: N) -> Self {
        Self::Pair(W::new(Self::Val(l)), W::new(Self::Val(r)))
    }

    fn reduce(&self) -> Self {
        use Num::{Val, Pair};

        let problem = match self {
            Val(x) if x >= 10 => Split(
        }
        self.clone()
    }
}

impl FromStr for Num {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        use Num::{Pair, Val};

        let mut stack = Vec::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                stack.push(Val(c.to_digit(10).unwrap()))
            } else if c == ']' {
                let r = W::new(stack.pop().unwrap());
                let l = W::new(stack.pop().unwrap());
                stack.push(Pair(l, r))
            }
        }

        assert!(stack.len() == 1);
        Ok(stack.pop().unwrap())
    }
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        Num::Pair(W::new(self), W::new(rhs)).reduce()
    }
}

impl Sum<Num> for Option<Num> {
    fn sum<I>(mut i: I) -> Self
    where
        I: Iterator<Item=Num>,
    {
        match i.next() {
            Some(first) => {
                Some(i.fold(first, |a, b| a + b))
            }
            None => None,
        }
    }
}

impl Num {
    fn mag(&self) -> N {
        use Num::{Pair, Val};

        match self {
            &Val(x) => x,
            Pair(left, right) => 3 * left.mag() + 2 * right.mag(),
        }
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
            assert_eq!(a.parse::<Num>().unwrap() + b.parse::<Num>().unwrap(), expected.parse::<Num>().unwrap());
        }
        assert_add("[1, 2]", "[3, 4]", "[[1, 2], [3, 4]]")

    }

    #[test]
    fn test_from_str() {
        use Num::Val;

        let s = "[1, 2]";
        let expected = Num::vp(1, 2);
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], 3]";
        let expected = Num::p(Num::vp(1, 2), Val(3));
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], [3, 4]]";
        let expected = Num::p(Num::vp(1, 2), Num::vp(3, 4));
        assert_eq!(s.parse(), Ok(expected));

        let s = "[[1, 2], [[3, 4], [5, [6, 7]]]]";
        let expected = Num::p(
            Num::vp(1, 2),
            Num::p(Num::vp(3, 4), Num::p(Val(5), Num::vp(6, 7))),
        );
        assert_eq!(s.parse(), Ok(expected));
    }

    #[test]
    fn test_mag() {
        use Num::Val;

        let p = Num::p(Num::vp(1, 2), Num::p(Num::vp(3, 4), Val(5)));
        assert_eq!(p.mag(), 143);

        let p = Num::p(
            Num::p(
                Num::p(Num::vp(0, 7), Val(4)),
                Num::p(Num::vp(7, 8), Num::vp(6, 0)),
            ),
            Num::vp(8, 1),
        );
        assert_eq!(p.mag(), 1384);
    }
}
