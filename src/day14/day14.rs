// -------------------------------------------------------
// Advent of Code 2023 - Day 14
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

enum Rock {
    Round,
    Cube,
    Nil,
}

impl Rock {
    fn from(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Cube,
            '.' => Self::Nil,
            _ => panic!("Invalid character for type 'Rock'"),
        }
    }
}

struct Platform {
    grid: Vec<Vec<Rock>>,
}

impl Platform {
    fn from(input: &str) -> Self {
        Self { grid: vec![] }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    0
}

fn pt2(input: &str) -> usize {
    0
}

pub fn day14() {
    let input = read_input("./src/day14/puzzle_input.txt");
    println!("Day 14:");
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....\
";

        assert_eq!(pt1(puzzle_input), 136);
    }

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // #.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.
    //
    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 400);
    //     }
}
