// -------------------------------------------------------
// Advent of Code 2023 - Day 17
// -------------------------------------------------------

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::read_input;

// -------------------------------------------------------
// Global Constants
// -------------------------------------------------------

const MAXSTEPS: u8 = 3;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nil,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Nil => Self::Nil,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
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

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Edge {
    from: Node,
    to: Node,
    weight: u32,
    direction: Direction,
}

impl Edge {
    fn from(from: Node, to: Node, weight: u32) -> Self {
        let direction = match from.row != to.row {
            true => match from.row < to.row {
                true => Direction::Down,
                false => Direction::Up,
            },
            false => match from.col < to.col {
                true => Direction::Right,
                false => Direction::Left,
            },
        };

        Self {
            from,
            to,
            weight,
            direction,
        }
    }
}

#[derive(Debug)]
struct Graph {
    source: Node,
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
    destination: Node,
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

        let source = Node::from(0, 0, grid[0][0]);
        let destination = Node::from(nrows - 1, ncols - 1, grid[nrows - 1][ncols - 1]);

        let mut nodes: HashSet<Node> = HashSet::new();
        let mut edges: HashSet<Edge> = HashSet::new();

        (0..nrows).for_each(|row| {
            (0..ncols).for_each(|col| {
                let u = Node::from(row, col, grid[row][col]);
                nodes.insert(u);

                let adjacent_coords = adjacent_coords(row, col, nrows, ncols);
                let adjacent_nodes = adjacent_coords
                    .iter()
                    .map(|(i, j)| Node::from(*i, *j, grid[*i][*j]));

                adjacent_nodes.for_each(|v| {
                    nodes.insert(v);

                    let e1 = Edge::from(u, v, v.loss);
                    let e2 = Edge::from(v, u, u.loss);
                    edges.insert(e1);
                    edges.insert(e2);
                })
            })
        });

        assert!(nodes.len() == nrows * ncols);

        Self {
            source,
            nodes,
            edges,
            destination,
        }
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

fn next_node(visited: &HashSet<Node>, losses: &HashMap<Node, u32>) -> Option<Node> {
    losses
        .iter()
        .filter(|(node, _)| !visited.contains(node))
        .min_by_key(|(_, &loss)| loss)
        .map(|(node, _)| *node)
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

// Something is off with my logic here
// Also look into using a priority queue rather than a hashmap to improve efficiency

fn pt1(input: &str) -> u32 {
    let mut graph = Graph::from(input);

    let mut visited: HashSet<Node> = HashSet::new();
    let mut losses: HashMap<Node, u32> = graph.nodes.iter().map(|node| (*node, u32::MAX)).collect();
    let mut paths: HashMap<Node, (Direction, u8)> = graph
        .nodes
        .iter()
        .map(|node| (*node, (Direction::Nil, 0)))
        .collect();

    if let Some(loss) = losses.get_mut(&graph.source) {
        *loss = 0;
    }

    if let Some(path) = paths.get_mut(&graph.source) {
        *path = (Direction::Nil, 0);
    }

    while let Some(current) = next_node(&visited, &losses) {
        graph.nodes.remove(&current);
        visited.insert(current);

        let (from_direction, steps) = paths[&current];

        graph
            .edges
            .iter()
            .filter_map(|edge| {
                match edge.from == current && edge.direction != from_direction.reverse() {
                    true => Some(*edge),
                    false => None,
                }
            })
            .for_each(|edge| {
                let to_direction = edge.direction;

                let loss_from = losses[&edge.from];

                match to_direction == from_direction {
                    true => match steps < MAXSTEPS {
                        true => {
                            if let Some(loss_to) = losses.get_mut(&edge.to) {
                                let loss = loss_from.saturating_add(edge.weight);

                                if loss < *loss_to {
                                    *loss_to = loss;
                                    paths.insert(edge.to, (to_direction, steps + 1));
                                }
                            }
                        }
                        false => (), // Do nothing, exceeds limit of 3 consecutive steps in the same direction
                    },
                    false => {
                        if let Some(loss_to) = losses.get_mut(&edge.to) {
                            let loss = loss_from.saturating_add(edge.weight);

                            if loss < *loss_to {
                                *loss_to = loss;
                                paths.insert(edge.to, (to_direction, 1));
                            }
                        }
                    }
                }
            });
    }

    losses[&graph.destination]
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
    //     use super::*;
    //
    //     #[test]
    //     fn test_pt1() {
    //         let puzzle_input = "\
    // 2413432311323
    // 3215453535623
    // 3255245654254
    // 3446585845452
    // 4546657867536
    // 1438598798454
    // 4457876987766
    // 3637877979653
    // 4654967986887
    // 4564679986453
    // 1224686865563
    // 2546548887735
    // 4322674655533\
    // ";
    //
    //         assert_eq!(pt1(puzzle_input), 102);
    //     }
    //
    //     #[test]
    //     fn test_pt2() {
    //         // let puzzle_input = "";
    //
    //         // assert_eq!(pt2(puzzle_input), 145);
    //     }
}
