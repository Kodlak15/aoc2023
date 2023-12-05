// -------------------------------------------------------
// Advent of Code 2023 - Day 3
// -------------------------------------------------------

use regex::Regex;

use crate::read_input;

// -------------------------------------------------------
// Custom data structures
// -------------------------------------------------------

#[allow(dead_code)]
struct ArrayBounds {
    rows: usize,
    cols: usize,
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
fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '0')
}

#[allow(dead_code)]
fn adjacent_coords(bounds: ArrayBounds, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = vec![];
    let row: i32 = row.try_into().unwrap();
    let col: i32 = col.try_into().unwrap();

    for i in row - 1..row + 2 {
        for j in col - 1..col + 2 {
            if i >= 0 && j >= 0 && bounds.validate(i.try_into().unwrap(), j.try_into().unwrap()) {
                adjacent.push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }

    adjacent
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let schematic: Vec<&str> = input.lines().collect();
    let bounds = ArrayBounds {
        rows: schematic.len(),
        cols: schematic[0].len(),
    };

    let adjacent = adjacent_coords(bounds, 9, 0);
    println!("Coords: {:?}", adjacent);

    0
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
