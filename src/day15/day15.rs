// -------------------------------------------------------
// Advent of Code 2023 - Day 15
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn hash(mut current: u32, c: char) -> u32 {
    current += c as u32;
    current *= 17;
    current %= 256;

    current
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let strings: Vec<String> = input.split(",").map(|s| s.to_string()).collect();

    strings
        .iter()
        .map(|s| {
            let mut current = 0;

            for c in s.chars() {
                current = match c {
                    '\n' => current,
                    _ => hash(current, c),
                }
            }

            current
        })
        .sum()
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
    fn test_hash() {
        let mut current = 0;

        for c in "HASH".chars() {
            println!("{:?} -> {:?}", c, c as u32);
            current = hash(current, c)
        }

        assert_eq!(current, 52);
    }

    #[test]
    fn test_pt1() {
        let puzzle_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(pt1(puzzle_input), 1320);
    }

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input, 3), 69);
    //     }
}
