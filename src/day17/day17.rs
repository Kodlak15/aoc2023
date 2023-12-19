// -------------------------------------------------------
// Advent of Code 2023 - Day 17
// -------------------------------------------------------

use std::rc::Rc;

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(PartialEq, Clone, Copy)]
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

#[derive(PartialEq)]
struct Edge {
    from: Node,
    to: Node,
    weight: u32,
}

impl Edge {
    fn from(from: Node, to: Node, weight: u32) -> Self {
        Self { from, to, weight }
    }
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Could not change character to u32!"))
                    .collect()
            })
            .collect();

        let nrows = grid.len();
        let ncols = grid[0].len();

        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();

        (0..nrows).for_each(|row| {
            (0..ncols).for_each(|col| {
                let u = Node::from(row, col, grid[row][col]);

                let adjacent_coords = adjacent_coords(row, col, nrows, ncols);
                let adjacent_nodes = adjacent_coords
                    .iter()
                    .map(|(i, j)| Node::from(*i, *j, grid[*i][*j]));

                if !nodes.contains(&u) {
                    nodes.push(u);
                }

                adjacent_nodes.for_each(|v| {
                    if !nodes.contains(&v) {
                        nodes.push(v)
                    }

                    let e1 = Edge::from(u, v, v.loss);
                    let e2 = Edge::from(v, u, u.loss);

                    if !edges.contains(&e1) {
                        edges.push(e1);
                    }

                    if !edges.contains(&e2) {
                        edges.push(e2);
                    }
                })
            })
        });

        Self { nodes, edges }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn adjacent_coords(row: usize, col: usize, nrows: usize, ncols: usize) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = Vec::new();

    if row > 0 {
        adjacent.push((row - 1, col))
    }

    if row < nrows - 1 {
        adjacent.push((row + 1, col))
    }

    if col > 0 {
        adjacent.push((row, col - 1))
    }

    if col < ncols - 1 {
        adjacent.push((row, col + 1))
    }

    adjacent
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let graph = Graph::from(input);

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
