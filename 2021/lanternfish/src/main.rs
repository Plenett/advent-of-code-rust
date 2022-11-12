use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let inputs = parse_input(read_file("input.txt"));

    println!("{:?}", inputs);

    // First Star
    let mut fishes = [0 as u32; 9];
    inputs.iter().for_each(|i| fishes[*i as usize] += 1);

    for _ in 0..80 {
        let reproduce = fishes[0];
        for i in 1..9 {
            fishes[i-1] = fishes[i];
        }

        fishes[6] += reproduce;
        fishes[8] = reproduce;
    }

    let mut counter = 0;
    fishes.iter().for_each(|f| counter += f);

    println!("The first answer id : {:?}", counter);

    // Second Star
    let mut fishes = [0 as u64; 9];
    inputs.iter().for_each(|i| fishes[*i as usize] += 1);

    for _ in 0..256 {
        let reproduce = fishes[0];
        for i in 1..9 {
            fishes[i-1] = fishes[i];
        }

        fishes[6] += reproduce;
        fishes[8] = reproduce;
    }

    let mut counter = 0;
    fishes.iter().for_each(|f| counter += f);

    println!("The second answer id : {:?}", counter);
}

fn parse_input(data: String) -> Vec<u8> {
    data.split(',').map(|n| n.parse::<u8>().unwrap()).collect()
}
