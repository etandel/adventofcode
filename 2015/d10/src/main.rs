use std::env;
use std::fs;
use std::path::Path;

fn next(prev: String) -> String {
    let mut out = String::with_capacity(2 * prev.len());

    let mut count = 0;
    let mut curr = None;

    for c in prev.chars() {
        match curr {
            Some(curr_c) if curr_c == c => {
                count += 1;
            }
            Some(curr_c) => {
                out.push_str(format!("{}", count).as_ref());
                out.push(curr_c);

                curr = Some(c);
                count = 1;
            }
            None => {
                curr = Some(c);
                count += 1;
            }
        }
    }

    out.push_str(format!("{}", count).as_ref());
    out.push(curr.unwrap());

    out
}

fn iter_n_times(input: String, n: usize) -> String {
    let mut res = input;

    for _ in 0..n {
        res = next(res);
    }

    res
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn part1() {
    let input = read_input("input.txt");
    println!("{}", iter_n_times(input, 40).len());
}

fn part2() {
    let input = read_input("input.txt");
    println!("{}", iter_n_times(input, 50).len());
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
    fn test_next() {
        assert_eq!(next("1".to_string()), "11".to_string());
        assert_eq!(next("11".to_string()), "21".to_string());
        assert_eq!(next("21".to_string()), "1211".to_string());
        assert_eq!(next("1211".to_string()), "111221".to_string());
        assert_eq!(next("111221".to_string()), "312211".to_string());
    }
}
