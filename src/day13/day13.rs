// -------------------------------------------------------
// Advent of Code 2023 - Day 13
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    fn from(pattern: &str) -> Self {
        let rows: Vec<String> = pattern.lines().map(|line| line.to_string()).collect();

        let mut cols: Vec<String> = Vec::new();
        cols.resize_with(rows[0].len(), String::new);

        for row in rows.iter().map(|row| row.chars()) {
            for i in 0..rows[0].len() {
                cols[i].push(
                    row.clone()
                        .nth(i)
                        .expect("Could not unpack character from row!"),
                );
            }
        }

        Self { rows, cols }
    }
}

struct Notes {
    patterns: Vec<Pattern>,
}

impl Notes {
    fn from(input: &str) -> Self {
        let patterns: Vec<Pattern> = input
            .split("\n\n")
            .map(|pattern| Pattern::from(pattern))
            .collect();

        // println!("Patterns: {:?}", patterns);

        Self { patterns }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn find_reflection(pattern: Vec<String>) -> usize {
    let mut i = 0;
    let mut j = 1;

    let mut reflection = 0;

    while j < pattern.len() {
        if pattern[i] == pattern[j] {
            reflection = j;
            break;
        }

        i += 1;
        j += 1;
    }

    if reflection == 0 {
        return 0;
    }

    while i > 0 && j < pattern.len() {
        if pattern[i] != pattern[j] {
            return 0;
        }

        i -= 1;
        j += 1;
    }

    reflection
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let notes = Notes::from(input);

    // For every pattern...
    // Find number of columns to the left of the vertical line of reflection
    // Find 100 * number of rows above the horizontal line of reflection
    // Sum the results (unsure if there can be more than one line of reflection)

    notes
        .patterns
        .iter()
        .map(|pattern| {
            find_reflection(pattern.cols.clone()) + (100 * find_reflection(pattern.rows.clone()))
        })
        .sum()
}

fn pt2(_input: &str) -> usize {
    0
}

pub fn day13() {
    let input = read_input("./src/day13/puzzle_input.txt");
    println!("Day 13:");
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#\
";

        assert_eq!(pt1(puzzle_input), 405);
    }
}
