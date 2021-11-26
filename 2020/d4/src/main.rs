use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Id {
    fields: Vec<String>,
}

const MANDATORY_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn validate_int_range(value: &str, min: u16, max: u16) -> bool {
    u16::from_str(value)
        .map(|v| v >= min && v <= max)
        .unwrap_or(false)
}

fn validate_pair(field: &str, value: &str) -> bool {
    match field {
        "byr" => validate_int_range(&value, 1920, 2002),

        "iyr" => validate_int_range(&value, 2010, 2020),

        "eyr" => validate_int_range(&value, 2020, 2030),

        "hgt" => {
            let (digits, suffix): (String, String) =
                value.chars().partition(|c| c.is_ascii_digit());

            match &suffix[..] {
                "cm" => validate_int_range(&digits, 150, 193),
                "in" => validate_int_range(&digits, 59, 76),
                _ => false,
            }
        }

        "hcl" => {
            let mut chars = value.chars();
            chars.next().map(|c| c == '#').unwrap_or(false)
                && chars.filter(|c| c.is_digit(16)).count() == 6
        }

        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&value[..]),

        "pid" => value.chars().filter(|c| c.is_digit(10)).count() == 9,

        "cid" => true,

        _ => false,
    }
}

impl Id {
    fn new() -> Self {
        Id {
            fields: Vec::with_capacity(8),
        }
    }

    fn push_field(&mut self, field: String) {
        self.fields.push(field);
    }

    fn is_valid(&self) -> bool {
        for field in MANDATORY_FIELDS {
            if !self.fields.contains(&field.to_string()) {
                return false;
            }
        }

        true
    }
}

fn parse_ids<P, F>(path: P, validator: F) -> Vec<Id>
where
    P: AsRef<Path>,
    F: Fn(&str, &str) -> bool,
{
    let mut ids = Vec::new();

    let mut current_id = Id::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        if line == "" {
            ids.push(current_id);
            current_id = Id::new();
        } else {
            for pair in line.split_ascii_whitespace() {
                let mut parts = pair.split(":").map(|s| s.to_string());

                let field = parts.next().unwrap();
                let value = parts.next().unwrap();

                if validator(&field, &value) {
                    current_id.push_field(field);
                }
            }
        }
    }

    ids
}

fn part1() {
    let ids = parse_ids("input.txt", |_, _| true);
    let count = ids.iter().filter(|i| i.is_valid()).count();
    println!("{}", count);
}

fn part2() {
    let ids = parse_ids("input.txt", validate_pair);
    let count = ids.iter().filter(|i| i.is_valid()).count();
    println!("{}", count);
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
    fn test_validate_pair() {
        assert!(validate_pair("byr", "2002"));
        assert!(!validate_pair("byr", "2003"));

        assert!(validate_pair("hgt", "60in"));
        assert!(validate_pair("hgt", "190cm"));
        assert!(!validate_pair("hgt", "190in"));
        assert!(!validate_pair("hgt", "190"));

        assert!(validate_pair("ecl", "brn"));
        assert!(!validate_pair("ecl", "wat"));

        assert!(validate_pair("pid", "000000001"));
        assert!(!validate_pair("pid", "0123456789"));
    }
}
