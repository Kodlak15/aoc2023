// -------------------------------------------------------
// Advent of Code 2023 - Day 14
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

enum Direction {
    North,
    West,
    South,
    East,
}

struct Platform {
    grid: Vec<Vec<char>>,
}

impl Platform {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for i in 0..self.grid.len() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = i;

                                while k > 0 && self.grid[k - 1][j] == '.' {
                                    self.grid[k][j] = '.';
                                    self.grid[k - 1][j] = 'O';
                                    k -= 1;
                                }
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::West => {
                for i in 0..self.grid.len() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = j;

                                while k > 0 && self.grid[i][k - 1] == '.' {
                                    self.grid[i][k] = '.';
                                    self.grid[i][k - 1] = 'O';
                                    k -= 1;
                                }
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::South => {
                for i in (0..self.grid.len()).rev() {
                    for j in 0..self.grid[i].len() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = i;

                                while k < self.grid.len() - 1 && self.grid[k + 1][j] == '.' {
                                    self.grid[k][j] = '.';
                                    self.grid[k + 1][j] = 'O';
                                    k += 1;
                                }
                            }
                            false => (),
                        }
                    }
                }
            }
            Direction::East => {
                for i in 0..self.grid.len() {
                    for j in (0..self.grid[i].len()).rev() {
                        match self.grid[i][j] == 'O' {
                            true => {
                                let mut k = j;

                                while k < self.grid[0].len() - 1 && self.grid[i][k + 1] == '.' {
                                    self.grid[i][k] = '.';
                                    self.grid[i][k + 1] = 'O';
                                    k += 1;
                                }
                            }
                            false => (),
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        // println!("Original:");
        // self.print();
        // println!("\n");
        // println!("Tilt North:");
        self.tilt(Direction::North);
        // self.print();
        // println!("\n");
        // println!("Tilt West:");
        self.tilt(Direction::West);
        // self.print();
        // println!("\n");
        // println!("Tilt South:");
        self.tilt(Direction::South);
        // self.print();
        // println!("\n");
        // println!("Tilt East:");
        self.tilt(Direction::East);
        // self.print();
        // println!("\n");
    }

    fn load(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let rounded = row
                    .iter()
                    .map(|c| *c)
                    .filter(|c| *c == 'O')
                    .collect::<Vec<char>>()
                    .len();

                rounded * (self.grid.len() - i)
            })
            .sum()
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let mut platform = Platform::from(input);
    platform.tilt(Direction::North);

    platform.load()
}

fn pt2(input: &str, cycles: usize) -> usize {
    let mut platform = Platform::from(input);

    for i in 0..cycles {
        if i % 1000 == 0 {
            println!("{:?} cycles completed...", i);
        }

        platform.cycle();
    }

    platform.load()
}

pub fn day14() {
    let input = read_input("./src/day14/puzzle_input.txt");
    println!("Day 14:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input, 1000000000));
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....\
";

        assert_eq!(pt1(puzzle_input), 136);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....\
";

        assert_eq!(pt2(puzzle_input, 3), 69);
    }
}
