use std::collections::HashSet;
use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let packet = parse_input(read_file("input.txt"));

    // First Star

    println!("The first answer is : {:?}", get_start_packet(&packet));

    // Second Star

    println!("The second answer is : {:?}", get_start_message(&packet));
}

fn parse_input(data: String) -> String {
    data
}

fn get_start_packet(packet: &String) -> usize {
    for i in 0..packet.len()-4 {
        let substr = &packet[i..i+4];
        let unique: HashSet<char> = HashSet::from_iter(substr.chars());
        if unique.len() == 4 {
            return i+4;
        }
    }
    return 0;
}

fn get_start_message(packet: &String) -> usize {
    for i in 0..packet.len()-14 {
        let substr = &packet[i..i+14];
        let unique: HashSet<char> = HashSet::from_iter(substr.chars());
        if unique.len() == 14 {
            return i+14;
        }
    }
    return 0;
}