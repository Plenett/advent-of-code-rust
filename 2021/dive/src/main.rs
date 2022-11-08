extern crate core;

use advent_utils::utils::read_file;
use regex::Regex;

fn main() {

    // Read & Parse
    let commands = parse_input(read_file("input.txt"));

    // First Star
    let mut pos = (0, 0);

    for c in &commands {
        pos = c.get_new_position(pos);
    }

    println!("The first answer id : {:?}", pos.0 * pos.1);

    // Second Star
    let mut pos = (0, 0);
    let mut depth: u32 = 0;

    for c in &commands {
        pos = c.get_new_position(pos);
        if let Command::FORWARD(X) = c {
            depth += pos.1 * X;
        }
    }

    println!("The second answer id : {:?}", pos.0 * depth);
}

fn parse_input(data: String) -> Vec<Command> {
    data.lines().map(|s| {
        Command::parse(s)
    }).collect()
}

#[derive(Debug, PartialEq)]
enum Command {
    FORWARD(u32),
    UP(u32),
    DOWN(u32)
}

impl Command {
    pub fn get_new_position(&self, pos: (u32, u32)) -> (u32, u32) {
        match self {
            Command::FORWARD(x) => (pos.0 + x, pos.1),
            Command::UP(x) => (pos.0, pos.1 - x),
            Command::DOWN(x) => (pos.0, pos.1 + x)
        }
    }

    pub fn parse(s: &str) -> Self {
        let regex = Regex::new(r"(forward|down|up)\s(\d+)").unwrap();
        let cap = regex.captures(s).expect("couldn't parse Command from string");
        let val: u32 = cap[2].parse().unwrap();

        match &cap[1] {
            "forward" => Command::FORWARD(val),
            "down" => Command::DOWN(val),
            "up" => Command::UP(val),
            &_ => {panic!("couldn't parse Command from string")}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Command;

    #[test]
    fn test_command() {
        let mut pos = (0, 0);

        pos = Command::FORWARD(5).get_new_position(pos);
        assert_eq!(pos, (5, 0));
        pos = Command::DOWN(5).get_new_position(pos);
        assert_eq!(pos, (5, 5));
        pos = Command::FORWARD(8).get_new_position(pos);
        assert_eq!(pos, (13, 5));
        pos = Command::UP(3).get_new_position(pos);
        assert_eq!(pos, (13, 2));

        assert_eq!(Command::parse("forward 5"), Command::FORWARD(5));
        assert_eq!(Command::parse("up 10"), Command::UP(10));
        assert_eq!(Command::parse("down 0"), Command::DOWN(0));
    }
}