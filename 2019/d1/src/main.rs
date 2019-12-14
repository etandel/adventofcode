use std::env;
use std::fs;
use std::str::FromStr;

fn calc_fuel_step(mass: i32) -> i32 {
    mass / 3 - 2
}


fn calc_module_fuel(mass: i32) -> i32 {
    let f = calc_fuel_step(mass);

    if f <= 0 {
        0
    } else {
        f + calc_module_fuel(f)
    }
}


fn calc_total_fuel(calculator: fn(i32) -> i32) -> i32 {
    let content = fs::read_to_string("input.txt").unwrap();
    content
        .lines()
        .map(|line| i32::from_str(line).unwrap())
        .map(calculator)
        .sum()
}


fn part1() {
    let result = calc_total_fuel(calc_fuel_step);
    println!("{}", result);
}


fn part2() {
    let result = calc_total_fuel(calc_module_fuel);
    println!("{}", result);
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_fuel_step() {
        assert_eq!(calc_fuel_step(12), 2);
        assert_eq!(calc_fuel_step(14), 2);
        assert_eq!(calc_fuel_step(1969), 654);
        assert_eq!(calc_fuel_step(100756), 33583);
    }

    #[test]
    fn test_calc_module_fuel() {
        assert_eq!(calc_module_fuel(12), 2);
        assert_eq!(calc_module_fuel(1969), 966);
        assert_eq!(calc_module_fuel(100756), 50346);
    }
}

