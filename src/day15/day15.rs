// -------------------------------------------------------
// Advent of Code 2023 - Day 15
// -------------------------------------------------------

use std::collections::{HashMap, VecDeque};

use crate::read_input;

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn hash(mut current: u32, c: char) -> u32 {
    current += c as u32;
    current *= 17;
    current %= 256;

    current
}

fn focusing_power(boxes: HashMap<u32, VecDeque<(&str, u32)>>) -> u32 {
    boxes
        .iter()
        .map(|(boxnum, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, (_, flen))| {
                    let n: u32 = lenses
                        .len()
                        .try_into()
                        .expect("Could not parse length of lenses as u32!");
                    let i: u32 = i.try_into().expect("Could not parse i as u32!");

                    (1 + boxnum) * (n - i) * flen
                })
                .sum::<u32>()
        })
        .sum()
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let strings: Vec<&str> = input.split(",").collect();

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

fn pt2(input: &str) -> u32 {
    let strings: Vec<&str> = input.split(",").collect();
    let mut boxes: HashMap<u32, VecDeque<(&str, u32)>> = HashMap::new();

    strings.iter().for_each(|s| {
        let op = match s.contains('-') || s.contains('=') {
            true => match s.contains('-') {
                true => '-',

                false => '=',
            },
            false => panic!("No operation found!"),
        };

        let step: Vec<&str> = s.split(op).collect();

        match op {
            '-' => {
                let label = step[0];
                let mut boxnum = 0;

                for c in label.chars() {
                    boxnum = hash(boxnum, c)
                }

                if let Some(boxn) = boxes.get_mut(&boxnum) {
                    if let Some(index) = boxn.iter().position(|(s, _)| s == &label) {
                        boxn.remove(index);
                    }
                }
            }
            '=' => {
                let label = step[0];
                let flen: u32 = step[1].parse().expect("Could not parse focal length!");
                let mut boxnum = 0;

                for c in label.chars() {
                    boxnum = hash(boxnum, c)
                }

                if let Some(boxn) = boxes.get_mut(&boxnum) {
                    if let Some(index) = boxn.iter().position(|(s, _)| s == &label) {
                        boxn[index] = (label, flen);
                    } else {
                        boxn.push_front((label, flen))
                    }
                } else {
                    let mut boxn: VecDeque<(&str, u32)> = VecDeque::new();
                    boxn.push_front((label, flen));
                    boxes.insert(boxnum, boxn);
                }
            }
            _ => panic!("Invalid operation!"),
        }
    });

    focusing_power(boxes)
}

pub fn day15() {
    let input = read_input("./src/day15/puzzle_input.txt");
    println!("Day 15:");
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
    fn test_hash() {
        let mut current = 0;

        for c in "HASH".chars() {
            current = hash(current, c)
        }

        assert_eq!(current, 52);
    }

    #[test]
    fn test_pt1() {
        let puzzle_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(pt1(puzzle_input), 1320);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(pt2(puzzle_input), 145);
    }
}
