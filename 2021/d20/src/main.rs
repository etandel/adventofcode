use std::env;
use std::fmt;
use std::fs;
use std::ops::{Add, Index, IndexMut, RangeInclusive};
use std::path::Path;

type Dim = isize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    y: Dim,
    x: Dim,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Point {
    fn new(y: Dim, x: Dim) -> Self {
        Self { x, y }
    }
}

fn iter_rows(r: RangeInclusive<Point>) -> impl Iterator<Item = impl Iterator<Item = Point>> {
    let &Point {
        y: start_y,
        x: start_x,
    } = r.start();
    let &Point { y: end_y, x: end_x } = r.end();

    (start_y..=end_y).map(move |y| (start_x..=end_x).map(move |x| Point { y, x }))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Image<T> {
    grid: Vec<T>,
    ncols: usize,
    nrows: usize,
    default: T,
}

impl<T> Image<T> {
    fn frame(&self, pos: Point) -> Vec<Point> {
        let deltas = [
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -1),
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, -1),
            Point::new(1, 0),
            Point::new(1, 1),
        ];

        deltas.iter().map(|d| *d + pos).collect()
    }

    fn top_left(&self) -> Point {
        Point::new(0, 0)
    }

    fn bottom_right(&self) -> Point {
        Point::new((self.nrows - 1) as Dim, (self.ncols - 1) as Dim)
    }

    fn subindex<'a>(&self, &Point { y, x }: &Point) -> Option<usize> {
        (y >= 0 && (y as usize) < self.nrows && x >= 0 && (x as usize) < self.ncols)
            .then(|| (y * self.ncols as Dim + x) as usize)
    }

    /// Starts at top-left (0, 0) and iterates over all points until bottom right (nrows, ncols)
    fn iter_points(&self) -> impl Iterator<Item = Point> {
        iter_rows(self.top_left()..=self.bottom_right()).flatten()
    }

    fn get<'a>(&'a self, p: &Point) -> Option<&'a T> {
        self.subindex(p).map(|i| &self.grid[i])
    }

    fn get_owned_with_default(&self, p: &Point) -> T
    where
        T: Copy,
    {
        *self.get(p).unwrap_or(&self.default)
    }

    fn set(&mut self, p: &Point, t: T) {
        let index = self.subindex(p).unwrap();
        self.grid[index] = t;
    }

    fn from_lines<I, SubI>(default: T, mut lines: I) -> Self
    where
        I: Iterator<Item = SubI>,
        SubI: IntoIterator<Item = T>,
    {
        let mut grid: Vec<T> = Vec::new();

        let mut ncols = 0;
        for iterrow in lines.by_ref() {
            for item in iterrow {
                ncols += 1;
                grid.push(item);
            }
            break;
        }

        grid.extend(lines.flatten());

        Self {
            nrows: grid.len() / ncols,
            ncols,
            grid,
            default,
        }
    }

    fn with_capacity(default: T, ncols: usize, nrows: usize) -> Self
    where
        T: Copy,
    {
        Image {
            ncols,
            nrows,
            default,
            grid: vec![default; ncols * nrows],
        }
    }
}

impl fmt::Display for Image<Pixel> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(
            for row in iter_rows(
                self.top_left() + Point::new(-2, -2)..=self.bottom_right() + Point::new(2, 2),
            ) {
                for p in row {
                    let c = if self.get_owned_with_default(&p) {
                        '#'
                    } else {
                        '.'
                    };
                    write!(f, "{}", c)?
                }
                write!(f, "{}", '\n')?
            },
        )
    }
}

type Pixel = bool;

fn process_frame(img: &Image<Pixel>, points: &[Point], algo: &[Pixel]) -> Pixel {
    let mut index: usize = 0;

    for p in points {
        index <<= 1;
        let bit = img.get_owned_with_default(p) as usize;
        index |= bit;
    }

    algo[index]
}

fn calc_default(def: Pixel, algo: &[Pixel]) -> Pixel {
    if def {
        algo[0b111111111]
    } else {
        algo[0]
    }
}

fn enhance_1(img: &Image<Pixel>, algo: &[Pixel]) -> Image<Pixel> {
    let new_default = calc_default(img.default, algo);

    let mut enhanced = Image::with_capacity(new_default, img.ncols + 4, img.nrows + 4);

    let start = {
        let Point { y, x } = img.top_left();
        Point { y: y - 2, x: x - 2 }
    };

    let end = {
        let Point { y, x } = img.bottom_right();
        Point { y: y + 2, x: x + 2 }
    };

    for p @ Point { y, x } in iter_rows(start..=end).flatten() {
        let index = Point::new(y + 2, x + 2);
        let newpixel = process_frame(img, &img.frame(p), algo);
        enhanced.set(&index, newpixel);
    }

    enhanced
}

fn enhance_n(mut img: Image<Pixel>, algo: &[Pixel], n: usize) -> Image<Pixel> {
    for _ in 0..n {
        img = enhance_1(&img, algo);
    }

    img
}

fn count_lit(img: &Image<Pixel>) -> usize {
    img.iter_points()
        .filter(|p| img.get_owned_with_default(p))
        .count()
}

fn parse_pixel(c: char) -> Pixel {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid pixel: {}", c),
    }
}

fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = Pixel> + 'a {
    line.chars().map(parse_pixel)
}

fn read_input<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn part1() {
    let input = read_input("input.txt");
    let mut lines = input.lines();

    let algo: Vec<Pixel> = parse_line(lines.next().unwrap()).collect();

    //skip empty line
    lines.next();

    let img = Image::from_lines(false, lines.map(parse_line));
    let final_image = enhance_n(img, &algo, 2);
    println!("{}", count_lit(&final_image));
}

fn part2() {
    let input = read_input("input.txt");
    let mut lines = input.lines();

    let algo: Vec<Pixel> = parse_line(lines.next().unwrap()).collect();

    //skip empty line
    lines.next();

    let img = Image::from_lines(false, lines.map(parse_line));
    let final_image = enhance_n(img, &algo, 50);
    println!("{}", count_lit(&final_image));
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
