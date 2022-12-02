use std::collections::HashSet;
use advent_utils::utils::read_file;
use crate::Node::{Big, End, Small, Start};

fn main() {

    // Read & Parse
    let mut vertices = parse_input(read_file("input.txt"));

    // First Star

    let all_path = get_all_paths(&vertices, &vec![Node::Start]);

    println!("The first answer is : {:?}", all_path.len());

    // Second Star

    let all_path = get_all_paths_2(&vertices, &vec![Node::Start]);

    println!("The second answer is : {:?}", all_path.len());
}

fn parse_input(data: String) -> Vec<Vertex> {
    data.lines().map(|l| {
        let nodes: Vec<&str> = l.split('-').collect();
        Vertex { a: Node::from_str(nodes[0]), b: Node::from_str(nodes[1]) }
    }).collect()
}

fn get_all_paths(vertices: &Vec<Vertex>, path: &Vec<Node>) -> Vec<Vec<Node>> {
    if *path.last().unwrap() == Node::End {
        return vec![path.clone()];
    }

    let mut paths = Vec::new();
    let last_node: Node = (*path.last().unwrap()).clone();

    for vertex in vertices {
        if vertex.contains(&last_node) {
            let next_node = (*vertex.return_other(&last_node).unwrap()).clone();

            match next_node {
                Start => continue,
                Small(_) => {
                    if path.contains(&next_node) {
                        continue;
                    }
                }
                _ => {}
            }

            let mut p = path.clone();
            p.push(next_node);

            let mut ps = get_all_paths(vertices, &p);

            paths.append(&mut ps);
        }
    }

    return paths;
}

fn get_all_paths_2(vertices: &Vec<Vertex>, path: &Vec<Node>) -> Vec<Vec<Node>> {
    if *path.last().unwrap() == Node::End {
        return vec![path.clone()];
    }

    let mut paths = Vec::new();
    let last_node: Node = (*path.last().unwrap()).clone();

    let mut uniq: HashSet<Node> = HashSet::new();
    let small_duplicates = !path.iter().all(move |x| {
        match x {
            Small(_) => {uniq.insert(x.clone())}
            _ => true
        }
    });

    for vertex in vertices {
        if vertex.contains(&last_node) {
            let next_node = (*vertex.return_other(&last_node).unwrap()).clone();

            match next_node {
                Start => continue,
                Small(_) => {
                    if path.contains(&next_node) && small_duplicates {
                        continue;
                    }
                }
                _ => {}
            }

            let mut p = path.clone();
            p.push(next_node);

            let mut ps = get_all_paths_2(vertices, &p);

            paths.append(&mut ps);
        }
    }

    return paths;
}


#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Node {
    fn from_str(str: &str) -> Self {
        match str {
            "start" => Start,
            "end" => End,
            n if n.chars().all(|c| c.is_uppercase()) => Big(String::from(str)),
            _ => Small(String::from(str))
        }
    }
}

#[derive(Debug)]
struct Vertex {
    a: Node,
    b: Node,
}

impl Vertex {
    fn contains(&self, node: &Node) -> bool {
        return self.a.eq(node) || self.b.eq(node);
    }

    fn return_other(&self, node: &Node) -> Option<&Node> {
        if !self.contains(node) {
            return None;
        }

        if self.a.eq(node) {
            return Some(&self.b);
        } else {
            return Some(&self.a);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Node, Vertex};

    #[test]
    fn test_contains() {
        let n1 = Node::Start;
        let n2 = Node::Big(String::from("A"));
        let n3 = Node::End;
        let n4 = Node::Start;
        let p = Vertex { a: n1, b: n2 };

        assert_eq!(p.contains(&n3), false);
        assert_eq!(p.contains(&n4), true);
    }

    #[test]
    fn test_return_other() {
        let n1 = Node::Start;
        let n2 = Node::Big(String::from("A"));
        let n3 = Node::End;
        let n4 = Node::Start;
        let p = Vertex { a: n1, b: n2 };

        assert_eq!(p.return_other(&n3), None);
        assert_eq!(p.return_other(&n4), Some(&Node::Big(String::from("A"))));
    }
}
