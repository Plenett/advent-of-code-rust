use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let map = parse_input(read_file("input.txt"));

    println!("{:?}", map);

    // First Star
    let mut risk_level: u32 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if is_low_point(&map, &(i, j)) {
                risk_level += *height as u32 + 1;
            }
        }
    }

    println!("The first answer id : {:?}", risk_level);

    // Second Star
    let mut basins_size: Vec<usize> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_low_point(&map, &(i, j)) {
                basins_size.push(get_basin(&map, &(i, j)).len());
            }
        }
    }

    basins_size.sort_by(|a, b| b.cmp(a));

    println!("The second answer id : {:?}", basins_size[0] * basins_size[1] * basins_size[2]);
}

fn parse_input(data: String) -> Vec<Vec<u8>> {
    data.lines().map(|s| {
        s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect()
}

fn get_neighbours(map: &Vec<Vec<u8>>, position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    if position.0 > 0 {
        neighbours.push((position.0 - 1, position.1));
    }
    if position.0 < map.len() - 1 {
        neighbours.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        neighbours.push((position.0, position.1 - 1));
    }
    if position.1 < map[position.0].len() - 1 {
        neighbours.push((position.0, position.1 + 1));
    }

    return neighbours
}

fn is_low_point(map: &Vec<Vec<u8>>, position: &(usize, usize)) -> bool {
    let val = map[position.0][position.1];

    for pos in get_neighbours(map, &position) {
        if val >= map[pos.0][pos.1] {
            return false;
        }
    }

    return true;
}

fn get_basin(map: &Vec<Vec<u8>>, position: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut basin = vec![position.clone()];
    let mut previous_len = 0;

    while basin.len() > previous_len {
        previous_len = basin.len();

        let mut new_basin: Vec<(usize, usize)> = Vec::new();
        for b in &basin {
            for n in get_neighbours(map, &b) {
                if !basin.contains(&n) && !new_basin.contains(&n) && map[n.0][n.1] < 9 {
                    new_basin.push(n);
                }
            }
        }

        basin.append(&mut new_basin);
    }

    return basin;
}

#[cfg(test)]
mod tests {
    use crate::{get_basin, is_low_point};

    #[test]
    fn test_is_low_point() {
        let map = vec![vec![2, 1, 9, 9, 9],
                       vec![3, 9, 8, 7, 8],
                       vec![9, 8, 5, 6, 7]];

        assert_eq!(is_low_point(&map, &(0, 1)), true);
        assert_eq!(is_low_point(&map, &(2, 2)), true);
        assert_eq!(is_low_point(&map, &(2, 3)), false);
    }

    #[test]
    fn test_get_basin() {
        let map = vec![vec![2, 1, 9, 9, 9],
            vec![3, 9, 8, 7, 8],
            vec![9, 8, 5, 6, 7]];

        assert_eq!(get_basin(&map, &(0, 1)), vec![(0, 1), (0, 0), (1, 0)]);
    }
}