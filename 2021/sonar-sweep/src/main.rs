use std::io;
use std::fs::File;
use std::io::BufRead;

fn main() {

    // Read File
    let file = File::open("input.txt")
        .expect("Should have been able to read the file");
    let lines = io::BufReader::new(file).lines();

    // Parse file
    let data: Vec<u32> = lines.map(|line|
        line.expect("cannot read").parse::<u32>().expect("cannot parse")
    ).collect();

    // First Star
    let mut nb_increased = 0;
    data.iter().zip(data.iter().skip(3))
        .for_each(|(a, b)| if b > a {nb_increased += 1;});

    println!("The first answer id : {:?}", nb_increased);

    // Second Star
    let mut nb_increased = 0;
    /* When we consider A+B+C < B+C+D, it is like considering only A < D. */
    data.iter().zip(data.iter().skip(3))
        .for_each(|(a, b)| if b > a {nb_increased += 1;});

    println!("The second answer id : {:?}", nb_increased);
}
