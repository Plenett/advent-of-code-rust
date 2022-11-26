use advent_utils::utils::read_file;

const SIZE: usize = 10;
const FLASH_LEVEL: u8 = 10;

fn main() {

    // Read & Parse
    let mut octopuses = parse_input(read_file("input.txt"));

    // First Star

    for t in 0..100 {
        octopuses.step();
    }

    println!("The first answer is : {:?}", octopuses.flashes);

    // Second Star
    let mut octopuses = parse_input(read_file("input.txt"));
    let mut t = 0;

    loop {
        t += 1;
        octopuses.step();

        if octopuses.have_synchronised() {
            break;
        }
    }

    println!("The second answer id : {:?}", t);
}

fn parse_input(data: String) -> Octopuses<SIZE> {
    let mut octopuses = [[0;SIZE];SIZE];
    for (i, l) in data.lines().enumerate() {
        for (j, n) in l.chars().enumerate() {
            octopuses[i][j] = n.to_digit(10).unwrap() as u8;
        }
    }

    return Octopuses { map: octopuses, flashes: 0 };
}

struct Octopuses<const N: usize> {
    map: [[u8; N]; N],
    flashes: u64
}

impl<const N: usize> Octopuses<N> {

    fn step(&mut self) {
        // Increase all octopuses
        for i in 0..N {
            for j in 0..N {
                self.map[i][j] += 1;
                if self.map[i][j] == FLASH_LEVEL {
                    self.flashes += 1;
                    self.flash((i, j));
                }
            }
        }

        // Reset octopuses that have flashed
        for i in 0..N {
            for j in 0..N {
                if self.map[i][j] >= FLASH_LEVEL {
                    self.map[i][j] = 0;
                }
            }
        }
    }

    fn flash(&mut self, pos: (usize, usize)) {
        for (i, j) in self.get_neighbours(pos) {
            self.map[i][j] += 1;
            if self.map[i][j] == FLASH_LEVEL {
                self.flashes += 1;
                self.flash((i, j));
            }
        }
    }

    fn get_neighbours(&self, pos: (usize, usize)) -> Vec<(usize,usize)>{
        let mut neighbours = Vec::new();

        let min_i = if pos.0 > 0 {pos.0-1} else {0};
        let min_j = if pos.1 > 0 {pos.1-1} else {0};
        let max_i = if pos.0 < N-1 {pos.0+2} else {N};
        let max_j = if pos.1 < N-1 {pos.1+2} else {N};

        for i in min_i..max_i {
            for j in min_j..max_j {
                if i == pos.0 && j == pos.1 {
                    continue;
                }
                neighbours.push((i, j));
            }
        }

        return neighbours;
    }

    fn have_synchronised(&self) -> bool {
        let val = self.map[0][0];

        for i in 0..N {
            for j in 0..N {
                if self.map[i][j] != val {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::Octopuses;

    #[test]
    fn test_get_neighbours() {
        let octopuses:Octopuses<10> = Octopuses{ map: [[0;10];10], flashes: 0 };

        assert_eq!(octopuses.get_neighbours((0, 0)),vec![(0,0), (0,1), (1,0), (1,1)]);
    }
}