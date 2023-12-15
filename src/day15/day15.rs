// -------------------------------------------------------
// Advent of Code 2023 - Day 15
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    0
}

fn pt2(input: &str, cycles: usize) -> usize {
    0
}

pub fn day15() {
    let input = read_input("./src/day15/puzzle_input.txt");
    println!("Day 15:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input, 1000));
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
";

        assert_eq!(pt1(puzzle_input), 136);
    }

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input, 3), 69);
    //     }
}
