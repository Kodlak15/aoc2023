// -------------------------------------------------------
// Advent of Code 2023 - Day 17
// -------------------------------------------------------

use std::rc::Rc;

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

struct Node {
    row: usize,
    col: usize,
    loss: u32,
}

impl Node {
    fn from(row: usize, col: usize, loss: u32) -> Self {
        Self { row, col, loss }
    }
}

struct Edge {
    from: Node,
    to: Node,
    weight: u32,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn from(input: &str) -> Self {
        let nodes: Vec<Node> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        let loss: u32 = c.to_digit(10).expect("Could not parse character as u32!");

                        Node::from(i, j, loss)
                    })
                    .collect::<Vec<Node>>()
            })
            .collect();

        Self {
            nodes: vec![],
            edges: vec![],
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    0
}

fn pt2(_input: &str) -> u32 {
    0
}

pub fn day17() {
    let input = read_input("./src/day17/puzzle_input.txt");
    println!("Day 17:");
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
    fn test_pt1() {
        let puzzle_input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533\
";

        assert_eq!(pt1(puzzle_input), 102);
    }

    #[test]
    fn test_pt2() {
        // let puzzle_input = "";

        // assert_eq!(pt2(puzzle_input), 145);
    }
}
