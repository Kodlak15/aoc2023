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

fn is_perfect_reflection(pattern: Vec<String>, coords: (usize, usize), smudged: bool) -> bool {
    let (mut i, mut j) = (coords.0, coords.1);

    match smudged {
        true => {
            // Number of mismatched characters
            let mut k = 0;

            loop {
                if pattern[i] != pattern[j] {
                    let ne = pattern[i]
                        .chars()
                        .zip(pattern[j].chars())
                        .filter(|(c1, c2)| c1 != c2)
                        .collect::<Vec<(char, char)>>()
                        .len();

                    if ne > 1 {
                        return false;
                    } else {
                        k += 1;
                    }

                    if k > 1 {
                        return false;
                    }
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

            // Return true if exactly one mismatched character found
            k == 1
        }
        false => {
            loop {
                if pattern[i] != pattern[j] {
                    return false;
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

            true
        }
    }
}

fn find_reflection(pattern: Vec<String>, smudged: bool) -> usize {
    let mut i = 0;
    let mut j = 1;

    while j < pattern.len() {
        match smudged {
            true => {
                let ne = pattern[i]
                    .chars()
                    .zip(pattern[j].chars())
                    .filter(|(c1, c2)| c1 != c2)
                    .collect::<Vec<(char, char)>>()
                    .len();

                if ne < 2 {
                    if is_perfect_reflection(pattern.clone(), (i, j), true) {
                        return j;
                    }
                }
            }
            false => {
                if pattern[i] == pattern[j] {
                    if is_perfect_reflection(pattern.clone(), (i, j), false) {
                        return j;
                    }
                }
            }
        }

        i += 1;
        j += 1;
    }

    0
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let notes = Notes::from(input);

    notes
        .patterns
        .iter()
        .map(|pattern| {
            let vn = find_reflection(pattern.cols.clone(), false);
            let hn = find_reflection(pattern.rows.clone(), false);

            assert!(vn == 0 || hn == 0);
            assert!(vn != 0 || hn != 0);

            vn + (100 * hn)
        })
        .sum()
}

fn pt2(input: &str) -> usize {
    let notes = Notes::from(input);

    notes
        .patterns
        .iter()
        .map(|pattern| {
            let vn = find_reflection(pattern.cols.clone(), true);
            let hn = find_reflection(pattern.rows.clone(), true);

            assert!(vn == 0 || hn == 0);
            assert!(vn != 0 || hn != 0);

            vn + (100 * hn)
        })
        .sum()
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

    #[test]
    fn test_pt2() {
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

        assert_eq!(pt2(puzzle_input), 400);
    }
}
