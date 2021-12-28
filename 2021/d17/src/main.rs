use std::cmp::max;
use std::env;
use std::fs;
use std::path::Path;

fn read_input<P>(path: P) -> (i32, i32, i32, i32)
where
    P: AsRef<Path>,
{
    let s = fs::read_to_string(path).unwrap();
    let mut parts = s.trim().split('=').skip(1);
    let mut x_bounds = parts.next().unwrap().split(",").next().unwrap().split("..").map(|s| s.parse().unwrap());
    let mut y_bounds = parts.next().unwrap().split("..").map(|s| s.parse().unwrap());

    (x_bounds.next().unwrap(), x_bounds.next().unwrap(), y_bounds.next().unwrap(), y_bounds.next().unwrap())
}

fn _position(v0: i32, t: i32) -> i32 {
    let v0_64 = v0 as f64;
    let t_64 = t as f64;

    ((-(t_64 * t_64) + (2.0 * v0_64 + 1.0) * t_64) / 2.0) as i32
}

fn max_x(v0: i32) -> i32 {
    _position(v0, v0)
}

fn position_x(v0: i32, t: i32) -> i32 {
    if t >= v0 {
        max_x(v0)
    } else {
        _position(v0, t)
    }
}

fn position_y(v0: i32, t: i32) -> i32 {
    _position(v0, t)
}

fn solve_t(v0: i32, s: i32) -> f64 {
    let b = (v0 as f64) + 0.5;
    b + (b * b - 2.0 * (s as f64)).sqrt()
}

fn t_range(v0: i32, lower: i32, upper: i32) -> impl Iterator<Item = i32> {
    let lower_t = solve_t(v0, lower);
    let upper_t = solve_t(v0, upper);

    if lower_t > upper_t {
        upper_t.ceil() as i32..=lower_t.floor() as i32
    } else {
        lower_t.ceil() as i32..=upper_t.floor() as i32
    }
}

fn calc_highest(v0y: i32) -> i32 {
    let t = (2 * v0y + 1) / 2;
    position_y(v0y, t)
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    count: usize,
    highest: i32,
}

fn solve(lower_x: i32, upper_x: i32, lower_y: i32, upper_y: i32) -> Stats {
    let mut count = 0;
    let mut highest = i32::MIN;

    for v0x in 0..=upper_x {
        for v0y in lower_y..=-lower_y {
            for t in t_range(v0y, lower_y, upper_y) {
                let pos_x = position_x(v0x, t);
                let pos_y = position_y(v0y, t);

                if lower_x <= pos_x && pos_x <= upper_x && lower_y <= pos_y && pos_y <= upper_y {
                    count += 1;
                    highest = max(highest, calc_highest(v0y));
                    break;
                }
            }
        }
    }

    Stats { count, highest }
}

fn part1() {
    let (lower_x, upper_x, lower_y, upper_y) = read_input("input.txt");
    //println!("{}", solve(20, 30, -10, -5).highest);
    //println!("{}", solve(79, 137, -176, -117).highest);
    println!("{}", solve(lower_x, upper_x, lower_y, upper_y).highest);
}

fn part2() {
    let (lower_x, upper_x, lower_y, upper_y) = read_input("input.txt");
    println!("{}", solve(lower_x, upper_x, lower_y, upper_y).count);
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
    #[test]
    fn test_() {}
}
