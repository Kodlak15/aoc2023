// -------------------------------------------------------
// Advent of Code 2023 - Day 16
// -------------------------------------------------------

use std::collections::VecDeque;

use crate::read_input;

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

enum Mirror {
    Positive,
    Negative,
}

impl Mirror {
    fn from(c: char) -> Self {
        match c {
            '/' => Self::Positive,
            '\\' => Self::Negative,
            _ => panic!("Invalid character for type 'Mirror'!"),
        }
    }
}

enum Splitter {
    Horizontal,
    Vertical,
}

impl Splitter {
    fn from(c: char) -> Self {
        match c {
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            _ => panic!("Invalid character for type 'Splitter'"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Beam {
    Up((usize, usize)),
    Down((usize, usize)),
    Left((usize, usize)),
    Right((usize, usize)),
}

impl Beam {
    fn coords(&self) -> (usize, usize) {
        match self {
            Beam::Up(coords) => *coords,
            Beam::Down(coords) => *coords,
            Beam::Left(coords) => *coords,
            Beam::Right(coords) => *coords,
        }
    }

    fn reflect(&self, mirror: char) -> Self {
        match Mirror::from(mirror) {
            Mirror::Positive => match self {
                Beam::Up(coords) => Beam::Right(*coords),
                Beam::Down(coords) => Beam::Left(*coords),
                Beam::Left(coords) => Beam::Down(*coords),
                Beam::Right(coords) => Beam::Up(*coords),
            },
            Mirror::Negative => match self {
                Beam::Up(coords) => Beam::Left(*coords),
                Beam::Down(coords) => Beam::Right(*coords),
                Beam::Left(coords) => Beam::Up(*coords),
                Beam::Right(coords) => Beam::Down(*coords),
            },
        }
    }

    fn split(&self, splitter: char) -> Vec<Self> {
        vec![]
    }
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_front(Beam::Right((0, 0)));

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut energized: Vec<(usize, usize)> = Vec::new();

    while !beams.is_empty() {
        beams = beams
            .iter()
            .filter_map(|beam| {
                let (row, col) = beam.coords();

                match row < nrows && col < ncols {
                    true => Some(*beam),
                    false => None,
                }
            })
            .collect()
    }

    0
}

fn pt2(_input: &str) -> u32 {
    0
}

pub fn day16() {
    let input = read_input("./src/day16/puzzle_input.txt");
    println!("Day 16:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------");
}

// -------------------------------------------------------
// Tests
// -------------------------------------------------------

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_pt1() {
        // let puzzle_input = "";

        // assert_eq!(pt1(puzzle_input), 1320);
    }

    // #[test]
    // fn test_pt2() {
    //     let puzzle_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    //
    //     assert_eq!(pt2(puzzle_input), 145);
    // }
}
