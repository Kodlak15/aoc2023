// -------------------------------------------------------
// Advent of Code 2023 - Day 14
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

struct Platform {
    grid: Vec<Vec<char>>,
}

impl Platform {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self { grid }
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

    for i in 0..platform.grid.len() {
        for j in 0..platform.grid[i].len() {
            match platform.grid[i][j] {
                'O' => {
                    let mut k = i;

                    while k > 0 && platform.grid[k - 1][j] == '.' {
                        platform.grid[k][j] = '.';
                        platform.grid[k - 1][j] = 'O';
                        k -= 1;
                    }
                }
                _ => (),
            }
        }
    }

    println!("{:?}\n\n", platform.grid);

    0
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
