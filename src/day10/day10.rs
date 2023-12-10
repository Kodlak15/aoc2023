// -------------------------------------------------------
// Advent of Code 2023 - Day 8
// -------------------------------------------------------

use std::rc::Rc;

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

enum Pipe {
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

// struct Pipe {
//     prev: Rc<Pipe>,
//     next: Rc<Pipe>,
// }

struct Diagram {
    pipes: Vec<Pipe>,
}

impl Diagram {
    fn from(input: &str) -> Self {
        Self { pipes: Vec::new() }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let diagram = Diagram::from(input);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_pt1() {
        let puzzle_input = "\
.....
.S-7.
.|.|.
.L-J.
.....\
";

        assert_eq!(pt1(puzzle_input), 4);
    }

    #[test]
    fn test1_pt1() {
        let puzzle_input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...\
";

        assert_eq!(pt1(puzzle_input), 4);
    }

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 6);
    //     }
}
