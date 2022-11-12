use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let (inputs, boards) = parse_input(read_file("input.txt"));

    // println!("{:?}", inputs);
    // println!("{:?}", boards);

    // First Star
    let mut i = 5; // we can't have a winning board before 5 inputs
    let mut result = 0;

    'outer: loop {
        for b in &boards {
            if b.is_winning(&inputs[0..i]) {
                result = b.get_score(&inputs[0..i]) * inputs[i - 1] as u32;
                break 'outer;
            }
        }
        i += 1;
    }

    println!("The first answer id : {:?}", result);

    // Second Star
    let mut boards_winning = vec![false;boards.len()];
    let mut i = 5;
    while boards_winning.contains(&false) {
        for (j, b) in boards.iter().enumerate() {
            if !boards_winning[j] && b.is_winning(&inputs[0..i]) {
                boards_winning[j] = true;
                result = b.get_score(&inputs[0..i]) * inputs[i-1] as u32;
            }
        }
        i += 1;
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
        for i in 0..5 {
            let mut row_winning = true;
            let mut col_winning = true;
            for j in 0..5 {
                row_winning &= inputs.contains(&self.grid[i][j]);
                col_winning &= inputs.contains(&self.grid[j][i]);
            }

            if col_winning || row_winning {
                return true;
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
