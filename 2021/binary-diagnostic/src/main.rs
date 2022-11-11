use advent_utils::utils::read_file;
use init_with::InitWith;

const SIZE: usize = 12;

fn main() {

    // Read & Parse
    let data = parse_input(read_file("input.txt"));

    // println!("{:?}", data);

    // First Star
    let majority = <[bool;SIZE]>::init_with_indices(|i| is_more_one(&data, i));

    let gamma = array_to_u32(majority.map(|x| if x {1} else {0}));
    let epsilon = array_to_u32(majority.map(|x| if !x {1} else {0}));

    println!("The first answer id : {:?}", gamma * epsilon);

    // Second Star
    let o2_rating = array_to_u32(get_o2_rating(&data, 0));
    let co2_rating = array_to_u32(get_co2_rating(&data, 0));

    println!("The second answer id : {:?}", o2_rating * co2_rating);
}

fn parse_input(data: String) -> Vec<[u8;SIZE]> {
    data.lines().map(|s| {
        let mut array: [u8;SIZE] = [0;SIZE];
        for i in 0..SIZE {
            array[i] = s.as_bytes()[i] - 48;
        }
        return array;
    }).collect()
}

fn array_to_u32<const N: usize>(array: [u8;N]) -> u32 {
    let mut val = 0;
    for i in 0..N {
        val <<= 1;
        val += array[i] as u32;
    }
    return val;
}

fn is_more_one<const N: usize>(data: &Vec<[u8; N]>, pos: usize) -> bool {
    let mut sum: usize = 0;

    data.iter().for_each(|a| sum += a[pos] as usize);

    return sum >= (data.len() + 1) / 2;
}

fn get_o2_rating(data: &Vec<[u8;SIZE]>, pos: usize) -> [u8;SIZE] {
    if data.len() == 1 {
        return data[0];
    }

    let more_one = is_more_one(&data, pos);
    let data = data.iter().filter(|a| (a[pos] == 1) == more_one).cloned().collect::<Vec<[u8;SIZE]>>();
    return get_o2_rating(&data, pos+1);
}

fn get_co2_rating(data: &Vec<[u8;SIZE]>, pos: usize) -> [u8;SIZE] {
    if data.len() == 1 {
        return data[0];
    }

    let more_one = is_more_one(&data, pos);
    let data = data.iter().filter(|a| (a[pos] == 1) != more_one).cloned().collect::<Vec<[u8;SIZE]>>();
    return get_co2_rating(&data, pos+1);
}

#[cfg(test)]
mod tests {
    use crate::array_to_u32;
    use crate::is_more_one;

    const SIZE: usize = 5;

    #[test]
    fn test_array_to_u32() {
        assert_eq!(array_to_u32::<SIZE>([0;SIZE]), 0);
        assert_eq!(array_to_u32::<SIZE>([0, 1, 1, 1, 1]), 15);
        assert_eq!(array_to_u32::<SIZE>([0, 0, 1, 0, 0]), 4);
    }

    #[test]
    fn test_is_more_one() {
        let data: Vec<[u8;SIZE]> = vec![[0, 0, 1, 0, 0], [1, 1, 1, 1, 0], [1, 0, 1, 1, 0], [1, 0, 1, 1, 1], [0, 1, 1, 1, 1]];

        assert_eq!(is_more_one::<SIZE>(&data, 0), true);
        assert_eq!(is_more_one::<SIZE>(&data, 1), false);
    }
}
