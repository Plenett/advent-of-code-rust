use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let (inputs, boards) = parse_input(read_file("input.txt"));

    // println!("{:?}", inputs);
    // println!("{:?}", boards);

    // First Star
    let mut winning = false;
    let mut i = 1;
    let mut result = 0;

    while !winning {
        for b in &boards {
            winning |= b.is_winning(&inputs[0..i]);
            if winning {
                result = b.get_score(&inputs[0..i]) * inputs[i-1] as u32;
                break;
            }
        }

        i += 1;
    }

    println!("The first answer id : {:?}", result);

    // Second Star
    for i in 5..inputs.len() {
        for b in &boards {
            if !b.is_winning(&inputs[0..i-1]) && b.is_winning(&inputs[0..i]) {
                result = b.get_score(&inputs[0..i]) * inputs[i-1] as u32;
                break;
            }
        }
    }

    println!("The second answer id : {:?}", result);
}

fn parse_input(data: String) -> (Vec<u8>, Vec<Board>) {
    let mut data = data.split("\r\n\r\n");

    let inputs = data.next().unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let boards = data.map(|s| Board::parse(s).unwrap()).collect();

    return (inputs, boards);
}

#[derive(Debug)]
struct Board {
    grid: [[u8;5];5]
}

impl Board {
    fn parse(string: &str) ->  Option<Self> {
        let mut board = Board { grid: [[0;5];5] };
        let mut i = 0;
        for (i, l) in string.lines().enumerate() {
            for (j, n) in l.split_whitespace().enumerate() {
                if i >= 5 || j >= 5 {
                    return None;
                }
                board.grid[i][j] = n.parse().unwrap()
            }
        }

        return Some(board);
    }

    fn is_winning(&self, inputs: &[u8]) -> bool {
        // Row
        for i in 0..5 {
            for j in 0..5 {
                if !inputs.contains(&self.grid[i][j]) {
                    break;
                }

                if j == 4 {
                    return true;
                }
            }
        }

        // column
        for j in 0..5 {
            for i in 0..5 {
                if !inputs.contains(&self.grid[i][j]) {
                    break;
                }

                if i == 4 {
                    return true;
                }
            }
        }

        return false
    }

    fn get_score(&self, inputs: &[u8]) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !inputs.contains(&self.grid[i][j]) {
                    sum += self.grid[i][j] as u32;
                }
            }
        }

        return sum;
    }
}
