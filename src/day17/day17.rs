// -------------------------------------------------------
// Advent of Code 2023 - Day 17
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    0
}

fn pt2(input: &str) -> u32 {
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
        let puzzle_input = "";

        // assert_eq!(pt1(puzzle_input), 1320);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "";

        assert_eq!(pt2(puzzle_input), 145);
    }
}
