// -------------------------------------------------------
// Advent of Code 2023 - Day 10
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

// Consider using a recursive function to traverse the diagram and find a solution
// - Start at starting pipe ('S')
// - Let rec be the recursive function that traverses the diagram and finds the result
// - Let i, j represent the cell in which the starting pipe resides
// - rec(i, j) = max(rec(i - 1, j), rec(i + 1, j), rec(i, j - 1), rec(i, j + 1))
// - If a nil value is encountered ('.') then the current path is not part of the loop
// - When the starting pipe is found again the loop has been closed, return the number of steps
// taken
// - I am inclined to say the result should be the total number of steps in the loop divided by 2,
// which would be halfway around the loop

#[derive(Debug)]
enum Pipe {
    Start,
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Nil,
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            'F' => Pipe::SouthEast,
            '7' => Pipe::SouthWest,
            '.' => Pipe::Nil,
            _ => panic!("Invalid character, no matching pipe!"),
        }
    }
}

struct Diagram {
    pipes: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Diagram {
    fn from(input: &str) -> Self {
        // let mut start: (usize, usize);

        let pipes: Vec<Vec<Pipe>> = input
            .lines()
            .map(|line| line.chars().map(|c| Pipe::from(c)).collect())
            .collect();

        println!("Pipes: {:?}", pipes);

        let start: (usize, usize) = input
            .lines()
            .enumerate()
            .filter_map(|(i, row)| {
                let m: Vec<(usize, usize)> = row
                    .chars()
                    .enumerate()
                    .filter_map(|(j, c)| match c == 'S' {
                        true => Some((i, j)),
                        false => None,
                    })
                    .collect();

                match m.len() > 0 {
                    true => Some(m[0]),
                    false => None,
                }
            })
            .collect::<Vec<(usize, usize)>>()[0];

        println!("Start: {:?}", start);

        Self {
            pipes,
            start: (0, 0),
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// fn adjacent_coords(coords: (usize, usize), dims: (usize, usize)) -> Vec<(usize, usize)> {
//     let adjacent: Vec<(usize, usize)> = Vec::new();
//
//     if i + 1 <
// }

fn find_loop() -> usize {
    0
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let mut diagram = Diagram::from(input);
    let dims = (diagram.pipes.len(), diagram.pipes[0].len());

    0
}

fn pt2(input: &str) -> usize {
    0
}

pub fn day10() {
    let input = read_input("./src/day08/puzzle_input.txt");
    println!("Day 10:");
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
.....
.S-7.
.|.|.
.L-J.
.....\
";

        assert_eq!(pt1(puzzle_input), 4);
    }

    #[test]
    fn test1_pt2() {
        let puzzle_input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...\
";

        assert_eq!(pt1(puzzle_input), 8);
    }

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 6);
    //     }
}
