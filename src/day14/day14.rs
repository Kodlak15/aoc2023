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
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                match self.grid[i][j] {
                    'O' => match direction {
                        Direction::North => {
                            let mut k = i;

                            while k > 0 && self.grid[k - 1][j] == '.' {
                                self.grid[k][j] = '.';
                                self.grid[k - 1][j] = 'O';
                                k -= 1;
                            }
                        }
                        Direction::West => {
                            let mut k = j;

                            while k > 0 && self.grid[j][k - 1] == '.' {
                                self.grid[i][k] = '.';
                                self.grid[i][k - 1] = 'O';
                                k -= 1;
                            }
                        }
                        Direction::South => {
                            let mut k = i;

                            while k < self.grid.len() - 1 && self.grid[k + 1][j] == '.' {
                                self.grid[k][j] = '.';
                                self.grid[k + 1][j] = 'O';
                                k += 1;
                            }
                        }
                        Direction::East => {
                            let mut k = j;

                            while k < self.grid[0].len() - 1 && self.grid[j][k + 1] == '.' {
                                self.grid[i][k] = '.';
                                self.grid[i][k + 1] = 'O';
                                k += 1;
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.grid
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                let rounded = row
                    .iter()
                    .map(|c| *c)
                    .filter(|c| *c == 'O')
                    .collect::<Vec<char>>()
                    .len();

                rounded * (i + 1)
            })
            .sum()
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

    // for i in 0..platform.grid.len() {
    //     for j in 0..platform.grid[i].len() {
    //         match platform.grid[i][j] {
    //             'O' => {
    //                 let mut k = i;
    //
    //                 while k > 0 && platform.grid[k - 1][j] == '.' {
    //                     platform.grid[k][j] = '.';
    //                     platform.grid[k - 1][j] = 'O';
    //                     k -= 1;
    //                 }
    //             }
    //             _ => (),
    //         }
    //     }
    // }

    platform.load()
}

fn pt2(input: &str) -> usize {
    0
}

pub fn day14() {
    let input = read_input("./src/day14/puzzle_input.txt");
    println!("Day 14:");
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

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // #.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.
    //
    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 400);
    //     }
}
