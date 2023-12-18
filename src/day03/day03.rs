// -------------------------------------------------------
// Advent of Code 2023 - Day 3
// -------------------------------------------------------

use regex::Regex;

use crate::read_input;

// -------------------------------------------------------
// Custom data structures
// -------------------------------------------------------

struct Schematic {
    numbers: Vec<(u32, usize, usize, usize)>,
    symbols: Vec<(usize, usize)>,
    gears: Vec<(usize, usize)>,
}

impl Schematic {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"\d+").expect("Could not unpack regular expression!");

        let numbers: Vec<(u32, usize, usize, usize)> = input
            .lines()
            .enumerate()
            .filter_map(|(row, line)| {
                let numbers: Vec<(u32, usize, usize, usize)> = re
                    .find_iter(line)
                    .map(|m| {
                        let n: u32 = m.as_str().parse().expect("Could not parse string as u32!");
                        let start = m.start();
                        let end = m.end();

                        (n, row, start, end)
                    })
                    .collect();

                match !numbers.is_empty() {
                    true => Some(numbers),
                    false => None,
                }
            })
            .flatten()
            .collect();

        let symbols: Vec<(usize, usize)> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                let symbols: Vec<(usize, usize)> = line
                    .chars()
                    .enumerate()
                    .filter_map(|(j, c)| match !c.is_digit(10) && c != '.' {
                        true => Some((i, j)),
                        false => None,
                    })
                    .collect();

                symbols
            })
            .collect();

        let gears: Vec<(usize, usize)> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                let symbols: Vec<(usize, usize)> = line
                    .chars()
                    .enumerate()
                    .filter_map(|(j, c)| match c == '*' {
                        true => Some((i, j)),
                        false => None,
                    })
                    .collect();

                symbols
            })
            .collect();

        Self {
            numbers,
            symbols,
            gears,
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn box_coords(row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
    (row.saturating_sub(1)..row + 2)
        .flat_map(|i| {
            (start.saturating_sub(1)..end + 1)
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let schematic = Schematic::from(input);

    schematic
        .numbers
        .iter()
        .filter_map(|(n, row, start, end)| {
            let coords = box_coords(*row, *start, *end);

            match coords.iter().any(|coord| schematic.symbols.contains(coord)) {
                true => Some(n),
                false => None,
            }
        })
        .sum()
}

#[allow(dead_code)]
fn pt2(_input: &str) -> u32 {
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

    #[test]
    fn test_pt1() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..\
    ";

        assert_eq!(pt1(input), 4361);
    }

    // #[test]
    // fn test_pt2() {
    //     unimplemented!()
    // }
}
