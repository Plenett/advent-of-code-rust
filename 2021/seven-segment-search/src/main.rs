use std::collections::HashMap;
use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let signals = parse_input(read_file("input.txt"));

    println!("{:?}", signals);

    // First Star
    let mut counter: u32 = 0;
    for (_, out) in &signals {
        for o in out {
            if is_simple_digit(o) {
                counter += 1;
            }
        }
    }

    println!("The first answer id : {:?}", counter);

    // Second Star
    let mut result: u32 = 0;
    for (sig, out) in &signals {
        let decode = decode_signal(sig);
        result += output_to_int(out, &decode);
    }

    println!("The second answer id : {:?}", result);
}

fn parse_input(data: String) -> Vec<(Vec<String>, Vec<String>)> {
    data.lines().map(|s| {
        let mut val = s.split(" | ");
        let signal = val.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();
        let output = val.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();

        return (signal, output)
    }).collect()
}

fn is_simple_digit(digit: &String) -> bool {
    match digit.len() {
        2 | 3 | 4 | 7 => true,
        _ => false
    }
}

fn sort_string(s: &String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn decode_signal(signal: &Vec<String>) -> HashMap<String, u8> {
    let mut decode: HashMap<String, u8> = HashMap::new();

    let mut one = String::new();
    let mut four = String::new();

    // Match simple digit
    for s in signal {
        let sorted = sort_string(s);

        match s.len() {
            2 => {
                decode.insert(sorted, 1);
                one = s.clone();
            }
            3 => {decode.insert(sorted, 7);}
            4 => {
                decode.insert(sorted, 4);
                four = s.clone();
            }
            7 => {decode.insert(sorted, 8);}
            _ => {}
        }
    }

    // Find others
    let bd: String = four.chars().filter(|c| !one.contains(*c)).collect();

    for s in signal {
        let sorted = sort_string(s);

        match s.len() {
            5 => {
                if one.chars().all(|c| s.contains(c)) {
                    decode.insert(sorted, 3);
                } else if bd.chars().all(|c| s.contains(c)) {
                    decode.insert(sorted, 5);
                } else {
                    decode.insert(sorted, 2);
                }
            }
            6 => {
                if !one.chars().all(|c| s.contains(c)) {
                    decode.insert(sorted, 6);
                } else if four.chars().all(|c| s.contains(c)) {
                    decode.insert(sorted, 9);
                } else {
                    decode.insert(sorted, 0);
                }
            }
            _ => {}
        }
    }

    return decode;
}

fn output_to_int(output: &Vec<String>, decode: &HashMap<String, u8>) -> u32 {
    let mut val = 0;

    for out in output {
        val *= 10;
        val += *decode.get(&sort_string(out)).unwrap() as u32;
    }

    return val;
}