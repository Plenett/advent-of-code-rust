use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let rounds = parse_input(read_file("input.txt"));

    // First Star
    let mut total_score: u32 = 0;

    for (opponent, player) in &rounds {
        total_score += player.get_shape_score() + player.get_result(opponent).get_result_score();
    }

    println!("The first answer is : {:?}", total_score);

    // Second Star
    let rounds = parse_input_2(read_file("input.txt"));

    let mut total_score: u32 = 0;

    for (opponent, result) in &rounds {
        let player_shape = Shape::what_should_i_play(opponent, result);
        total_score += player_shape.get_shape_score() + result.get_result_score();
    }

    println!("The second answer is : {:?}", total_score);
}

fn parse_input(data: String) -> Vec<(Shape, Shape)> {
    data.lines().map(|l| {
        let shapes: Vec<&str> = l.split(' ').collect();
        (Shape::from_str(shapes[0]).unwrap(), Shape::from_str(shapes[1]).unwrap())
    }).collect()
}

fn parse_input_2(data: String) -> Vec<(Shape, Result)> {
    data.lines().map(|l| {
        let shapes: Vec<&str> = l.split(' ').collect();
        (Shape::from_str(shapes[0]).unwrap(), Result::from_str(shapes[1]).unwrap())
    }).collect()
}

#[derive(Debug)]
enum Result {
    Win,
    Draw,
    Lose
}

impl Result {
    fn from_str(str: &str) -> Option<Result> {
        match str {
            "X" => Some(Result::Lose),
            "Y" => Some(Result::Draw),
            "Z" => Some(Result::Win),
            _ => None
        }
    }

    fn get_result_score(&self) -> u32 {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(str: &str) -> Option<Shape> {
        match str {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None
        }
    }

    fn what_should_i_play(opponent: &Shape, result: &Result) -> Shape {
        match opponent {
            Shape::Rock => {
                match result {
                    Result::Win => Shape::Paper,
                    Result::Draw => Shape::Rock,
                    Result::Lose => Shape::Scissors
                }
            }
            Shape::Paper => {
                match result {
                    Result::Win => Shape::Scissors,
                    Result::Draw => Shape::Paper,
                    Result::Lose => Shape::Rock
                }
            }
            Shape::Scissors => {
                match result {
                    Result::Win => Shape::Rock,
                    Result::Draw => Shape::Scissors,
                    Result::Lose => Shape::Paper
                }
            }
        }
    }

    fn get_result(&self, opponent: &Self) -> Result {
        match self {
            Shape::Rock => {
                match opponent {
                    Shape::Rock => Result::Draw,
                    Shape::Paper => Result::Lose,
                    Shape::Scissors => Result::Win
                }
            }
            Shape::Paper => {
                match opponent {
                    Shape::Rock => Result::Win,
                    Shape::Paper => Result::Draw,
                    Shape::Scissors => Result::Lose,
                }
            }
            Shape::Scissors => {
                match opponent {
                    Shape::Rock => Result::Lose,
                    Shape::Paper => Result::Win,
                    Shape::Scissors => Result::Draw
                }
            }
        }
    }

    fn get_shape_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}