// -------------------------------------------------------
// Advent of Code 2023 - Day 10
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Pipe {
    Start,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Nil,
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            'F' => Pipe::SouthEast,
            '7' => Pipe::SouthWest,
            '.' => Pipe::Nil,
            _ => panic!("Invalid character, no matching pipe!"),
        }
    }
}

struct Diagram {
    pipes: Vec<Vec<Pipe>>,
    start: (usize, usize),
    dims: (usize, usize),
}

impl Diagram {
    fn from(input: &str) -> Self {
        let pipes: Vec<Vec<Pipe>> = input
            .lines()
            .map(|line| line.chars().map(|c| Pipe::from(c)).collect())
            .collect();

        let start: (usize, usize) = input
            .lines()
            .enumerate()
            .filter_map(|(i, row)| {
                let m: Vec<(usize, usize)> = row
                    .chars()
                    .enumerate()
                    .filter_map(|(j, c)| match c == 'S' {
                        true => Some((i, j)),
                        false => None,
                    })
                    .collect();

                match m.len() > 0 {
                    true => Some(m[0]),
                    false => None,
                }
            })
            .collect::<Vec<(usize, usize)>>()[0];

        let dims = (pipes.len(), pipes[1].len());

        Self { pipes, start, dims }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn adjacent_coords(
    pipe: Pipe,
    coords: (usize, usize),
    dims: (usize, usize),
    visited: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = Vec::new();

    let (i, j) = coords; // i = row index, j = col index
    let (m, n) = dims; // m = num rows, n = num cols

    match pipe {
        Pipe::Start => {
            if i + 1 < m {
                adjacent.push((i + 1, j));
            }

            if i > 0 {
                adjacent.push((i - 1, j));
            }

            if j + 1 < n {
                adjacent.push((i, j + 1));
            }

            if j > 0 {
                adjacent.push((i, j - 1));
            }
        }
        Pipe::Horizontal => {
            if j + 1 < n {
                adjacent.push((i, j + 1));
            }
            if j > 0 {
                adjacent.push((i, j - 1));
            }
        }
        Pipe::Vertical => {
            if i + 1 < m {
                adjacent.push((i + 1, j));
            }

            if i > 0 {
                adjacent.push((i - 1, j));
            }
        }
        Pipe::NorthEast => {
            if i + 1 < m {
                adjacent.push((i + 1, j));
            }

            if j + 1 < n {
                adjacent.push((i, j + 1));
            }
        }
        Pipe::NorthWest => {
            if i + 1 < m {
                adjacent.push((i + 1, j));
            }

            if j > 0 {
                adjacent.push((i, j - 1));
            }
        }
        Pipe::SouthEast => {
            if i > 0 {
                adjacent.push((i - 1, j));
            }

            if j + 1 < n {
                adjacent.push((i, j + 1));
            }
        }
        Pipe::SouthWest => {
            if i > 0 {
                adjacent.push((i - 1, j));
            }

            if j > 0 {
                adjacent.push((i, j - 1));
            }
        }
        Pipe::Nil => {}
    }

    adjacent
        .iter()
        .filter_map(|adj| match !visited.contains(adj) {
            true => Some(*adj),
            false => None,
        })
        .collect::<Vec<(usize, usize)>>()
}

fn find_loop(
    pipes: &Vec<Vec<Pipe>>,
    coords: (usize, usize),
    dims: (usize, usize),
    visited: Vec<(usize, usize)>,
) -> usize {
    let pipe = pipes[coords.0][coords.1];
    let adjacent = adjacent_coords(pipe, coords, dims, visited.to_vec());

    println!("Adjacent: {:?}", adjacent);

    0
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let diagram = Diagram::from(input);

    let mut coords = diagram.start;
    let dims = diagram.dims;
    let mut visited: Vec<(usize, usize)> = Vec::new();

    find_loop(&diagram.pipes, coords, dims, visited);

    loop {
        break;
    }

    0
}

fn pt2(input: &str) -> usize {
    0
}

pub fn day10() {
    let input = read_input("./src/day08/puzzle_input.txt");
    println!("Day 10:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

// -------------------------------------------------------
// Tests
// -------------------------------------------------------

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test1_pt1() {
//         let puzzle_input = "\
// .....
// .S-7.
// .|.|.
// .L-J.
// .....\
// ";
//
//         assert_eq!(pt1(puzzle_input), 4);
//     }
//
//     #[test]
//     fn test1_pt2() {
//         let puzzle_input = "\
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...\
// ";
//
//         assert_eq!(pt1(puzzle_input), 8);
//     }

//     #[test]
//     fn test_pt2() {
//         let puzzle_input = "\
// ";
//
//         assert_eq!(pt2(puzzle_input), 6);
//     }
// }
