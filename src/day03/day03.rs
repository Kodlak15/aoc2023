// -------------------------------------------------------
// Advent of Code 2023 - Day 3
// -------------------------------------------------------

use regex::Regex;

use crate::read_input;

// -------------------------------------------------------
// Global constants
// -------------------------------------------------------

const SYMBOLS: &str = "`~!@#$%^&*()_+=";

// -------------------------------------------------------
// Custom data structures
// -------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct ArrayBounds {
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

#[allow(dead_code)]
impl ArrayBounds {
    fn validate(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

#[allow(dead_code)]
fn adjacent_coords(
    bounds: ArrayBounds,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> Vec<Coordinates> {
    let mut adjacent: Vec<Coordinates> = vec![];

    for i in row.saturating_sub(1)..row + 2 {
        for j in col_start.saturating_sub(1)..col_end + 2 {
            if bounds.validate(i, j) {
                adjacent.push(Coordinates { row: i, col: j });
            }
        }
    }

    adjacent
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let schematic: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let bounds = ArrayBounds {
        rows: schematic.len(),
        cols: schematic[0].len(),
    };

    let re = Regex::new(r"\d+").unwrap();
    // TODO

    0
}

#[allow(dead_code)]
fn pt2(_input: &str) -> u32 {
    // TODO
    0
}

pub fn day03() {
    let input = read_input("./src/day03/puzzle_input.txt");
    println!("Day 3:");
    println!("Part 1: {}", pt1(&input));
    // println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

// -------------------------------------------------------
// Tests
// -------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    //    fn test_pt1() {
    //        let input = "\
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..\
    // ";
    //
    //        assert_eq!(pt1(input), 4361);
    //    }

    // #[test]
    // fn test_pt2() {
    //     unimplemented!()
    // }
}
