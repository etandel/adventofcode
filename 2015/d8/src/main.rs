use std::env;
use std::fs;
use std::path::Path;

type Parsed = Vec<u8>;

fn decode(s: &str) -> Parsed {
    let mut out = Parsed::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        let next = match c {
            '"' => None,
            '\\' => match chars.next() {
                Some('"') => Some(b'"'),
                Some('\\') => Some(b'\\'),
                Some('x') => match (chars.next(), chars.next()) {
                    (Some(x1), Some(x2)) => {
                        let hex = format!("{}{}", x1, x2);
                        u8::from_str_radix(&hex[..], 16).ok()
                    }
                    _ => panic!("Incomplete hex escape sequence at end of str {}", s),
                },
                Some(x) => panic!("Invalid escaped char {} in str {}", x, s),
                None => panic!("Incomplete escape sequence at end of str {}", s),
            },
            x => Some(x as u8),
        };

        if let Some(next_c) = next {
            out.push(next_c);
        }
    }

    out
}

fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2 + 2);

    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            x => out.push(x),
        }
    }
    out.push('"');

    out
}

fn read_strings<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

fn part1() {
    let res: usize = read_strings("input.txt")
        .iter()
        .map(|s| (s, decode(s)))
        .map(|(raw, parsed)| raw.len() - parsed.len())
        .sum();

    println!("{}", res);
}

fn part2() {
    let res: usize = read_strings("input.txt")
        .iter()
        .map(|s| (s, encode(s)))
        .map(|(raw, encoded)| encoded.len() - raw.len())
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
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("\"\""), "".bytes().collect::<Vec<u8>>());
        assert_eq!(decode("\"abc\""), "abc".bytes().collect::<Vec<u8>>());
        assert_eq!(
            decode("\"aaa\\\"aaa\""),
            "aaa\"aaa".bytes().collect::<Vec<u8>>()
        );
        assert_eq!(decode("\"\\x27\""), "\x27".bytes().collect::<Vec<u8>>());
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("\"\""), "\"\\\"\\\"\"");

        assert_eq!(encode("\"abc\""), "\"\\\"abc\\\"\"");
        assert_eq!(
            encode("\"\\x27\""),
            "\" \\\" \\\\x27 \\\" \"".replace(" ", "")
        );
    }
}
