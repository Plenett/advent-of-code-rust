use advent_utils::utils::read_file;

fn main() {

    // Read & Parse
    let crabs = parse_input(read_file("input.txt"));

    println!("{:?}", crabs);

    // First Star
    let mut min_fuel: u32 = get_linear_fuel_cost(&crabs, 0);
    for i in 1..1000 {
        if min_fuel > get_linear_fuel_cost(&crabs, i) {
            min_fuel = get_linear_fuel_cost(&crabs, i);
        }
    }

    println!("The first answer id : {:?}", min_fuel);

    // Second Star
    let mut min_fuel: u32 = get_real_fuel_cost(&crabs, 0);
    for i in 1..1000 {
        if min_fuel > get_real_fuel_cost(&crabs, i) {
            min_fuel = get_real_fuel_cost(&crabs, i);
        }
    }

    println!("The second answer id : {:?}", min_fuel);
}

fn parse_input(data: String) -> Vec<u32> {
    data.split(',').map(|n| n.parse::<u32>().unwrap()).collect()
}

fn get_linear_fuel_cost(crabs: &Vec<u32>, position: u32) -> u32 {
    let mut cost = 0;

    for c in crabs {
        if *c > position {
            cost += *c - position;
        } else {
            cost += position - *c;
        }
    }

    return cost;
}

fn get_real_fuel_cost(crabs: &Vec<u32>, position: u32) -> u32 {
    let mut cost = 0;

    for c in crabs {
        let n = if *c > position {
            *c - position
        } else {
            position - *c
        };
        cost += (n * (n+1))/2;
    }

    return cost;
}

#[cfg(test)]
mod tests {
    use crate::{get_linear_fuel_cost, get_real_fuel_cost};

    #[test]
    fn test_get_linear_fuel_cost() {
        let crabs: Vec<u32> = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(get_linear_fuel_cost(&crabs, 2), 37);
        assert_eq!(get_linear_fuel_cost(&crabs, 3), 39);
    }

    #[test]
    fn test_get_real_fuel_cost() {
        let crabs: Vec<u32> = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(get_real_fuel_cost(&crabs, 2), 206);
        assert_eq!(get_real_fuel_cost(&crabs, 5), 168);
    }
}