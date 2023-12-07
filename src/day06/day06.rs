// -------------------------------------------------------
// Advent of Code 2023 - Day 6
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Math
// -------------------------------------------------------

// Let x = the number of whole ms the button is held down
// let n = the number of whole ms in the race
// speed(x) = x mm/ms
// time(n, x) = n - x ms
// distance(n, x) = speed(x) * time(n, x) mm = x * (n - x) mm = x*n - x**2 mm
// d/dx distance(n, x) = n - 2*x
// Global maxima at x = n/2

// -------------------------------------------------------
// Custom data structures
// -------------------------------------------------------

struct Races {
    lengths: Vec<u32>,
    records: Vec<u32>,
}

impl Race {
    fn from(input: &str) -> Self {}
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    0
}

#[allow(dead_code)]
fn pt2(input: &str) -> u32 {
    0
}

pub fn day06() {
    let input = read_input("./src/day06/puzzle_input.txt");
    println!("Day 6:");
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
\
";

        // assert_eq!(pt1(puzzle_input), 35);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
\
";

        // assert_eq!(pt2(puzzle_input), 46);
    }
}
