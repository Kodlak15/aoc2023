// -------------------------------------------------------
// Advent of Code 2023 - Day 8
// -------------------------------------------------------

use std::collections::HashMap;

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

struct Map {
    instructions: String,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn from(input: &str) -> Self {
        let instructions = input
            .lines()
            .nth(0)
            .expect("Could not find the instructions!")
            .to_string();

        let nodes: HashMap<String, (String, String)> = input
            .lines()
            .skip(2)
            .map(|line| {
                let splitln: Vec<&str> = line.split(" = ").collect();
                let name = splitln[0].to_string();
                let pair: Vec<&str> = splitln[1].split(", ").collect();
                let left = pair[0].to_string().replace("(", "");
                let right = pair[1].to_string().replace(")", "");

                (name, (left, right))
            })
            .collect();

        Self {
            instructions,
            nodes,
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn next_instruction(instructions: &String, n: usize) -> char {
    instructions
        .chars()
        .nth(n % instructions.len())
        .expect("Failed to get next instruction!")
}

fn least_common_multiple(v: &mut Vec<usize>) -> usize {
    let v0 = v.clone();

    while !v.iter().all(|x| *x == v[0]) {
        let min = v.iter().min().expect("Failed to unpack min of v!");
        let idx = v
            .iter()
            .position(|x| x == min)
            .expect("Could not unpack index of min of v!");
        v[idx] += v0[idx];
    }

    v[0]
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let map = Map::from(input);

    let mut n = 0;
    let mut instruction = next_instruction(&map.instructions, n);
    let mut node = "AAA".to_string();
    let end_node = "ZZZ".to_string();

    while node != end_node {
        node = match instruction {
            'L' => map.nodes[&node].0.clone(),
            'R' => map.nodes[&node].1.clone(),
            _ => panic!("Invalid instruction!"),
        };

        n += 1;
        instruction = next_instruction(&map.instructions, n);
    }

    n
}

fn pt2(input: &str) -> usize {
    let map = Map::from(input);

    let nodes: Vec<String> = map
        .nodes
        .keys()
        .filter_map(|node| match node.ends_with('A') {
            true => Some(node.to_string()),
            false => None,
        })
        .collect();

    let mut v: Vec<usize> = nodes
        .iter()
        .map(|node| {
            let mut n = 0;
            let mut instruction = next_instruction(&map.instructions, n);
            let mut current_node = node.clone();

            while !current_node.ends_with('Z') {
                current_node = match instruction {
                    'L' => map.nodes[&current_node].0.clone(),
                    'R' => map.nodes[&current_node].1.clone(),
                    _ => panic!("Invalid instruction!"),
                };

                n += 1;
                instruction = next_instruction(&map.instructions, n);
            }

            n
        })
        .collect();

    least_common_multiple(&mut v)
}

pub fn day08() {
    let input = read_input("./src/day08/puzzle_input.txt");
    println!("Day 8:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

// -------------------------------------------------------
// Tests
// -------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_pt1() {
        let puzzle_input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)\
";

        assert_eq!(pt1(puzzle_input), 2);
    }

    #[test]
    fn test2_pt1() {
        let puzzle_input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)\
";

        assert_eq!(pt1(puzzle_input), 6);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)\
";

        assert_eq!(pt2(puzzle_input), 6);
    }
}
