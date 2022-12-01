use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let calories = parse_input(read_file("input.txt"));

    // First Star
    let mut max_elf: u32 = 0;
    for elf in &calories {
        let sum = elf.iter().sum();
        if sum > max_elf {
            max_elf = sum;
        }
    }

    println!("The first answer is : {:?}", max_elf);

    // Second Star
    let mut elves: Vec<u32> = calories.iter().map(|e| e.iter().sum()).collect();

    elves.sort_by(|a, b| b.cmp(a));

    println!("The second answer is : {:?}", elves[0] + elves[1] + elves[2]);
}

fn parse_input(data: String) -> Vec<Vec<u32>> {
    let mut calories = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for line in data.lines(){
        if line.is_empty() {
            calories.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }

    return calories;
}