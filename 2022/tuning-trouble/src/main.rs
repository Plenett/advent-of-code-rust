use std::collections::HashSet;
use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let packet = parse_input(read_file("input.txt"));

    // First Star

    println!("The first answer is : {:?}", get_start_packet::<4>(&packet));

    // Second Star

    println!("The second answer is : {:?}", get_start_packet::<14>(&packet));
}

fn parse_input(data: String) -> String {
    data
}

fn get_start_packet<const N: usize>(packet: &String) -> usize {
    for i in 0..packet.len()-N {
        let substr = &packet[i..i+N];
        let unique: HashSet<char> = HashSet::from_iter(substr.chars());
        if unique.len() == N {
            return i+N;
        }
    }
    return 0;
}