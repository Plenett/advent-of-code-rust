use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let lines = parse_input(read_file("input.txt"));

    // println!("{:?}", lines);

    // First Star
    let mut score = 0;
    for line in &lines {
        let mut chunk_stack: Vec<Chunk> = Vec::new();
        for c in line {
            if c.closing {
                let expected = chunk_stack.pop();
                if expected.is_none() || expected.unwrap().chunk_type != c.chunk_type {
                    score += c.get_illegal_score();
                    break;
                }
            } else {
                chunk_stack.push((*c).clone());
            }
        }
    }

    println!("The first answer is : {:?}", score);

    // Second Star
    let mut scores: Vec<u64> = Vec::new();
    'outer: for line in &lines {
        let mut chunk_stack: Vec<Chunk> = Vec::new();
        for c in line {
            if c.closing {
                let expected = chunk_stack.pop();
                if expected.is_none() || expected.unwrap().chunk_type != c.chunk_type {
                    continue 'outer;
                }
            } else {
                chunk_stack.push((*c).clone());
            }
        }

        if chunk_stack.is_empty() {
            continue;
        }

        let mut completion_score: u64 = 0;
        for c in chunk_stack.iter().rev() {
            completion_score = completion_score*5 + c.get_completion_score() as u64;
        }
        scores.push(completion_score);
    }

    scores.sort();

    println!("The second answer is : {:?}", scores[scores.len()/2]);
}

fn parse_input(data: String) -> Vec<Vec<Chunk>> {
    data.lines().map(|s| {
        s.chars().map(|c| Chunk::parse(c).unwrap()).collect()
    }).collect()
}


#[derive(Debug, PartialEq, Clone)]
enum ChunkType {
    Parenthesis,
    Bracket,
    Brace,
    Chevron
}

#[derive(Debug, PartialEq, Clone)]
struct Chunk {
    closing: bool,
    chunk_type: ChunkType
}

impl Chunk {
    fn get_illegal_score(&self) -> u32 {
        match self.chunk_type {
            ChunkType::Parenthesis => 3,
            ChunkType::Bracket => 57,
            ChunkType::Brace => 1197,
            ChunkType::Chevron => 25137
        }
    }

    fn get_completion_score(&self) -> u32 {
        match self.chunk_type {
            ChunkType::Parenthesis => 1,
            ChunkType::Bracket => 2,
            ChunkType::Brace => 3,
            ChunkType::Chevron => 4
        }
    }

    fn parse(c: char) -> Option<Self> {
        match c {
            '(' => Some(Chunk { closing: false, chunk_type: ChunkType::Parenthesis }),
            ')' => Some(Chunk { closing: true, chunk_type: ChunkType::Parenthesis }),
            '[' => Some(Chunk { closing: false, chunk_type: ChunkType::Bracket }),
            ']' => Some(Chunk { closing: true, chunk_type: ChunkType::Bracket }),
            '{' => Some(Chunk { closing: false, chunk_type: ChunkType::Brace }),
            '}' => Some(Chunk { closing: true, chunk_type: ChunkType::Brace }),
            '<' => Some(Chunk { closing: false, chunk_type: ChunkType::Chevron }),
            '>' => Some(Chunk { closing: true, chunk_type: ChunkType::Chevron }),
            _ => None
        }
    }
}