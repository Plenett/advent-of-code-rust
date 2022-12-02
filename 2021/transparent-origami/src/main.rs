use std::collections::HashSet;
use advent_utils::utils::read_file;
use regex::Regex;


fn main() {

    // Read & Parse
    let (points, folds) = parse_input(read_file("input.txt"));

    // First Star
    let first_fold = remove_duplicate(points.iter().map(|p| folds[0].fold_point(p)).collect());

    println!("The first answer is : {:?}", first_fold.len());

    // Second Star
    let mut points = points;

    for fold in folds {
        points = remove_duplicate(points.iter().map(|p| fold.fold_point(p)).collect())
    }

    println!("The second answer is :");
    print_paper(&points);
}

fn parse_input(data: String) -> (Vec<(u32, u32)>, Vec<Fold>) {
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<Fold> = Vec::new();

    let re_fold = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    let re_point = Regex::new(r"^(\d+),(\d+)$").unwrap();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        if re_fold.is_match(line) {
            let caps = re_fold.captures(line).unwrap();
            match &caps[1] {
                "x" => {folds.push(Fold::X(caps[2].parse().unwrap()))}
                "y" => {folds.push(Fold::Y(caps[2].parse().unwrap()))}
                _ => {}
            }
        } else {
            let caps = re_point.captures(line).unwrap();
            points.push((caps[1].parse().unwrap(), caps[2].parse().unwrap()));
        }

    }

    return (points, folds);
}

fn remove_duplicate(points: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    HashSet::<(u32, u32)>::from_iter(points).into_iter().collect()
}

fn print_paper(paper: &Vec<(u32, u32)>) {
    let max_x = paper.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = paper.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    for j in 0..max_y+1 {
        for i in 0..max_x+1 {
            if paper.contains(&(i, j)) {
                print!("x");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    fn fold_point(&self, point: &(u32, u32)) -> (u32, u32) {
        match self {
            Fold::X(f) => {
                if point.0 > *f {
                    (2*f - point.0, point.1)
                } else {
                    (point.0, point.1)
                }
            }
            Fold::Y(f) => {
                if point.1 > *f {
                    (point.0, 2*f - point.1)
                } else {
                    (point.0, point.1)
                }
            }
        }
    }
}