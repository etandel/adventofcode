use std::collections::VecDeque;
use std::env;
use std::fs;
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use std::fmt;

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

    fn reduce(mut self) -> Self {
        loop {
            let mut reduced = false;
            let mut open_count = 0;

            for i in 0..self.vals.len() {
                match self.vals[i] {
                    Open => open_count += 1,
                    Close => open_count -= 1,

                    Val(left) if open_count > 4 => {
                        //dbg!(&self.vals, i, open_count, left);

                        // sum to left
                        for k in (0..i).rev() {
                            if let Val(v) = self.vals[k] {
                                self.vals[k] = Val(v + left);
                                break;
                            }
                        }

                        // sum to right
                        if let Val(right) = self.vals[i + 1] {
                            for k in i + 2..self.vals.len() {
                                if let Val(v) = self.vals[k] {
                                    self.vals[k] = Val(v + right);
                                    break;
                                }
                            }
                        } else {
                            panic!("Missing right!");
                        }

                        // replace current with 0
                        self.vals[i - 1] = Val(0);
                        self.vals.remove(i);
                        self.vals.remove(i);
                        self.vals.remove(i);

                        reduced = true;
                        break;
                    }

                    Val(x) if x >= 10 => {
                        let d = x as f32 / 2.0;

                        self.vals[i] = Open;
                        self.vals.insert(i + 1, Close);
                        self.vals.insert(i + 1, Val(d.ceil() as N));
                        self.vals.insert(i + 1, Val(d.floor() as N));

                        reduced = true;
                        break;
                    }

                    _ => {}
                }
            }

            if !reduced {
                break;
            }
        }

        self
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

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in 0..self.vals.len() {
            match self.vals[i] {
                Open => {
                    write!(f, "{}", '[')?;
                },

                Close => {
                    write!(f, "{}", ']')?;

                    if let Some(Open | Val(_)) = self.vals.get(i + 1) {
                        write!(f, "{}", ',')?;
                    }
                },

                Val(x) => {
                    write!(f, "{}", x)?;

                    if let Some(Open | Val(_)) = self.vals.get(i + 1) {
                        write!(f, "{}", ',')?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        println!("{}", &self);
        println!("{}", &rhs);

        let r = Self::p(self, rhs).reduce();

        println!("{}", &r);
        println!("{}", "=========");

        r
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
        let mut stack = Vec::new();
        for entry in &self.vals {
            match entry {
                Close => {
                    // because of stack, right comes first when popping
                    let v = stack.pop().unwrap() * 2 + stack.pop().unwrap() * 3;
                    stack.push(v);
                }
                &Val(x) => {
                    stack.push(x);
                }
                _ => {}
            }
        }

        stack.pop().unwrap()
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
    let num = nums.into_iter().sum::<Option<Num>>().unwrap();

    println!("{}", num);
    let res = num.mag();
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

    fn assert_reduce(n: &str, expected: &str) {
        assert_eq!(
            n.parse::<Num>().unwrap().reduce(),
            expected.parse::<Num>().unwrap()
        );
    }

    #[test]
    fn test_reduce_explode() {
        assert_reduce("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_reduce("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_reduce("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_reduce(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_reduce_split() {
        assert_eq!(Num::vp(10, 2).reduce(), "[[5, 5], 2]".parse().unwrap());
        assert_eq!(Num::vp(11, 2).reduce(), "[[5, 6], 2]".parse().unwrap());

        let p = Num::p(Num::vp(10, 2), Num::vp(2, 11));
        assert_eq!(p.reduce(), "[[[5, 5], 2], [2, [5, 6]]]".parse().unwrap());
    }

    fn assert_add(a: &str, b: &str, expected: &str) {
        assert_eq!(
            a.parse::<Num>().unwrap() + b.parse::<Num>().unwrap(),
            expected.parse::<Num>().unwrap()
        );
    }

    #[test]
    fn test_add_simple() {
        assert_add("[1, 2]", "[3, 4]", "[[1, 2], [3, 4]]")
    }

    #[test]
    fn test_add_reduce() {
        assert_add("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1, 1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
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

    #[test]
    fn test_sum() {
        fn assert_sum(s: &str, expected: &str) {
            let nums: Vec<Num> = s.lines().map(|l| l.parse().unwrap()).collect();
            let res: Option<Num> = nums.into_iter().sum();
            assert_eq!(res.unwrap(), expected.parse().unwrap());
        }

//        let s = "[1,1]
//                 [2,2]
//                 [3,3]
//                 [4,4]";
//        assert_sum(s, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
//
//        let s = "[1,1]
//                 [2,2]
//                 [3,3]
//                 [4,4]
//                 [5,5]";
//        assert_sum(s, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
//
//        let s = "[1,1]
//                 [2,2]
//                 [3,3]
//                 [4,4]
//                 [5,5]
//                 [6,6]";
//        assert_sum(s, "[[[[5,0],[7,4]],[5,5]],[6,6]]");

        let s = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                 [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                 [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                 [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                 [7,[5,[[3,8],[1,4]]]]
                 [[2,[2,2]],[8,[8,1]]]
                 [2,9]
                 [1,[[[9,3],9],[[9,0],[0,7]]]]
                 [[[5,[7,4]],7],1]
                 [[[[4,2],2],6],[8,7]]";

        assert_sum(s, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }
}
