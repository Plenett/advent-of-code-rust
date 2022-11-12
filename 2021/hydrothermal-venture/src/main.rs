use std::collections::HashMap;
use std::ops::Add;
use advent_utils::utils::read_file;
use regex::Regex;

fn main() {

    // Read & Parse
    let lines = parse_input(read_file("input.txt"));

    // println!("{:?}", lines);

    // First Star
    let mut covered: HashMap<Point, u8> = HashMap::new();
    for l in &lines {
        if l.is_diagonal() {
            continue
        }

        for p in l.get_points_on_line() {
            let val = covered.entry(p).or_insert(0);
            *val += 1;
        }
    }

    let mut counter: u32 = 0;
    covered.iter().for_each(|(_, v)| if *v > 1 {counter += 1;});

    println!("The first answer id : {:?}", counter);

    // Second Star
    let mut covered: HashMap<Point, u8> = HashMap::new();
    for l in &lines {
        for p in l.get_points_on_line() {
            let val = covered.entry(p).or_insert(0);
            *val += 1;
        }
    }

    let mut counter: u32 = 0;
    covered.iter().for_each(|(_, v)| if *v > 1 {counter += 1;});

    println!("The second answer id : {:?}", counter);
}

fn parse_input(data: String) -> Vec<Line> {
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    data.lines().map(|l| {
        let cap = regex.captures(l).unwrap();
        Line {a: Point {x: cap[1].parse::<i32>().unwrap(), y: cap[2].parse::<i32>().unwrap()},
            b: Point {x: cap[3].parse::<i32>().unwrap(), y: cap[4].parse::<i32>().unwrap()} }
    }).collect()
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.a.x != self.b.x && self.a.y != self.b.y
    }

    fn get_direction(&self) -> Point {
        let x = match self.b.x - self.a.x {
            d if d > 0 => 1,
            d if d == 0 => 0,
            _ => -1
        };

        let y = match self.b.y - self.a.y {
            d if d > 0 => 1,
            d if d == 0 => 0,
            _ => -1
        };

        Point { x, y }
    }

    fn get_points_on_line(&self) -> Vec<Point> {
        let mut point = self.a;
        let dir = self.get_direction();
        let mut points = vec![point];

        loop {
            point = point + dir;
            points.push(point);
            if point == self.b {
                break
            }
        }

        return points;
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;
    use crate::Line;

    #[test]
    fn test_point() {
        let p1 = Point {x: 10, y: -2};
        let p2 = Point {x: -3, y: 5};

        assert_eq!(p1 + p2, Point {x: 7, y: 3});
    }

    #[test]
    fn test_is_diagonal() {
        let l1 = Line { a: Point { x: 645, y: 570 }, b: Point { x: 517, y: 570 } };
        let l2 = Line { a: Point { x: 945, y: 914 }, b: Point { x: 98, y: 67 } };

        assert_eq!(l1.is_diagonal(), false);
        assert_eq!(l2.is_diagonal(), true);
    }

    #[test]
    fn test_get_points_on_line() {
        let l1 = Line { a: Point { x: 0, y: 9 }, b: Point { x: 5, y: 9 } };
        let p1 = vec![Point { x: 0, y: 9 }, Point { x: 1, y: 9 },
                      Point { x: 2, y: 9 }, Point { x: 3, y: 9 },
                      Point { x: 4, y: 9 }, Point { x: 5, y: 9 }];
        assert_eq!(l1.get_points_on_line(), p1);

        let l2 = Line { a: Point { x: 8, y: 2 }, b: Point { x: 6, y: 0 } };
        let p2 = vec![Point { x: 8, y: 2 }, Point { x: 7, y: 1 }, Point { x: 6, y: 0 }];
        assert_eq!(l2.get_points_on_line(), p2);
    }
}
