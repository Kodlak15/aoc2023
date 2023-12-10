// -------------------------------------------------------
// Advent of Code 2023 - Day 10
// -------------------------------------------------------

use std::collections::HashMap;

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
}

impl Diagram {
    fn from(input: &str) -> Self {
        let pipes: Vec<Vec<Pipe>> = input
            .lines()
            .map(|line| line.chars().map(|c| Pipe::from(c)).collect())
            .collect();

        let mut grid: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        for i in 0..pipes.len() {
            for j in 0..pipes[0].len() {
                match pipes[i][j] {
                    Pipe::Start => grid
                        .insert((i, j), Vec::new())
                        .expect("Failed to insert vector into grid!"),
                    Pipe::Vertical => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i, j + 1));
                        grid[&(i, j)].push((i, j - 1));
                    }
                    Pipe::Horizontal => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i + 1, j));
                        grid[&(i, j)].push((i - 1, j));
                    }
                    Pipe::NorthEast => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i, j + 1));
                        grid[&(i, j)].push((i + 1, j));
                    }
                    Pipe::NorthWest => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i, j + 1));
                        grid[&(i, j)].push((i - 1, j));
                    }
                    Pipe::SouthEast => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i, j - 1));
                        grid[&(i, j)].push((i + 1, j));
                    }
                    Pipe::SouthWest => {
                        grid.insert((i, j), Vec::new())
                            .expect("Failed to insert vector into grid!");
                        grid[&(i, j)].push((i, j - 1));
                        grid[&(i, j)].push((i - 1, j));
                    }
                    Pipe::Nil => grid.insert((i, j), Vec::new()),
                };
            }
        }

        println!("Grid: {:?}", grid);

        println!("Pipes: {:?}", pipes);

        Self { pipes: Vec::new() }
    }
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let diagram = Diagram::from(input);

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
