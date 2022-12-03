use std::collections::HashSet;
use advent_utils::utils::read_file;


fn main() {

    // Read & Parse
    let rucksacks = parse_input(read_file("input.txt"));

    // First Star
    let mut score: u32 = 0;

    for r in &rucksacks {
        let len = r.len();
        let first = &r[0..len/2];
        let second = &r[len/2..len];

        // !("total: {:?} -- first: {} -- second: {:?}", r, first, second);

        let unique: HashSet<char> = HashSet::from_iter(first.chars());
        for c in second.chars() {
            if unique.contains(&c) {
                // println!("{}: {}", c, letter_to_priority(c));
                score += letter_to_priority(c) as u32;
                break;
            }
        }
    }

    println!("The first answer is : {:?}", score);

    // Second Star
    let mut score: u32 = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let common = common_letters(&rucksacks[i], &rucksacks[i+1]);
        let common = common_letters(&rucksacks[i+2], &common);

        score += letter_to_priority(common.chars().next().unwrap()) as u32;
    }

    println!("The second answer is : {:?}", score);
}

fn parse_input(data: String) -> Vec<String> {
    data.lines().map(|s| s.to_string()).collect()
}

fn letter_to_priority(letter: char) -> u8 {
    if letter.is_lowercase() {
        letter as u8 - 96
    } else {
        letter as u8 - 64 + 26
    }
}

fn common_letters(first: &String, second: &String) -> String {
    let mut common_letters: HashSet<char> = HashSet::new();

    let unique: HashSet<char> = HashSet::from_iter(first.chars());
    for c in second.chars() {
        if unique.contains(&c) {
            common_letters.insert(c);
        }
    }

    return String::from_iter(common_letters);
}