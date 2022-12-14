use advent_utils::utils::read_file;
use regex::Regex;

const SIZE: usize = 9;

fn main() {

    // Read & Parse
    let (stacks_data, moves) = parse_input(read_file("input.txt"));

    // First Star
    let mut stacks = stacks_data.clone();
    for m in &moves {
        stacks = move_stack(stacks, m);
    }

    println!("The first answer is : {:?}", get_code(&stacks));

    // Second Star
    let mut stacks = stacks_data.clone();
    for m in &moves {
        stacks = move_stack_all(stacks, m);
    }

    println!("The second answer is : {:?}", get_code(&stacks));
}

fn parse_input(data: String) -> (Vec<Stack>, Vec<Move>) {
    let mut data = data.split("\r\n\r\n");

    (parse_stacks(data.next().unwrap()), parse_moves(data.next().unwrap()))
}

fn parse_moves(str: &str) -> Vec<Move> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    str.lines().map(|s| {
        let cap = re.captures(s).unwrap();
        Move { source: cap[2].parse().unwrap(), dest: cap[3].parse().unwrap(), amount: cap[1].parse().unwrap() }
    }).collect()
}

fn parse_stacks(str: &str) -> Vec<Stack> {
    let mut stacks = vec![Stack::new();SIZE];
    let re = Regex::new(r"\[([A-Z])\]").unwrap();

    str.lines().for_each(|l| {
        let mut caps = re.captures_iter(l);
        for cap in caps {
            let idx = (cap.get(1).unwrap().start() - 1)/4;
            stacks[idx].insert(0, cap[1].chars().next().unwrap())
        }
    });

    return stacks;
}

fn move_stack(mut stacks: Vec<Stack>, m: &Move) -> Vec<Stack> {
    let len = stacks[m.source-1].len() - m.amount;
    let mut elems: Vec<char> = stacks[m.source-1].drain(len..).collect();
    elems.reverse();
    stacks[m.dest-1].append(&mut elems);
    return stacks
}

fn move_stack_all(mut stacks: Vec<Stack>, m: &Move) -> Vec<Stack> {
    let len = stacks[m.source-1].len() - m.amount;
    let mut elems: Vec<char> = stacks[m.source-1].drain(len..).collect();
    stacks[m.dest-1].append(&mut elems);
    return stacks
}

fn get_code(stacks: &Vec<Stack>) -> String {
    let mut code: Vec<char> = Vec::new();
    for s in stacks {
        code.push(s.last().copied().unwrap());
    }
    return code.into_iter().collect();
}

type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    source: usize,
    dest: usize,
    amount: usize,
}