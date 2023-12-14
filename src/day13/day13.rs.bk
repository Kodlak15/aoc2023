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

        Self { patterns }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// Note: there can be more than one possible line of reflection! Only one will be a perfect line
// of reflection. If one is found to be imperfect, find the next one and try that one. See first
// pattern in input for an example.
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
        // println!("Pattern: {:?}", pattern);
        // println!("Reflection: {:?}", reflection);
        // println!("-------------------------------------------------------");

        return 0;
    }

    loop {
        println!("pattern[{:?}]: {:?}", i, pattern[i]);
        println!("pattern[{:?}]: {:?}", j, pattern[j]);

        if pattern[i] != pattern[j] {
            // println!("Pattern: {:?}", pattern);
            // println!("Reflection: {:?}", 0);
            // println!("-------------------------------------------------------");

            return 0;
        }

        if i == 0 {
            break;
        }

        if j == pattern.len() - 1 {
            break;
        }

        i -= 1;
        j += 1;
    }

    // println!("Pattern: {:?}", pattern);
    // println!("Reflection: {:?}", reflection);
    println!("-------------------------------------------------------");

    reflection
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let notes = Notes::from(input);

    let res: Vec<usize> = notes
        .patterns
        .iter()
        .map(|pattern| {
            let vn = find_reflection(pattern.cols.clone());
            let hn = 100 * find_reflection(pattern.rows.clone());

            // Assert that one, but not both, of the values is 0
            // This is analogous to asserting there is exactly one vertical or one horizontal line
            // of reflection
            assert!(vn == 0 || hn == 0);
            assert!(vn != 0 || hn != 0);

            vn + hn
        })
        .collect();

    // println!("Res: {:?}", res);

    res.iter().sum()
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
    fn test_pt1() {
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
