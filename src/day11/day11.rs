// -------------------------------------------------------
// Advent of Code 2023 - Day 11
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug, Clone)]
struct Universe {
    image: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Universe {
    fn from(input: &str) -> Self {
        let image: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let mut row_not_empty: Vec<usize> = Vec::new();
        let mut col_not_empty: Vec<usize> = Vec::new();

        for i in 0..image.len() {
            for j in 0..image[0].len() {
                if image[i][j] == '#' {
                    if !row_not_empty.contains(&i) {
                        row_not_empty.push(i);
                    }

                    if !col_not_empty.contains(&j) {
                        col_not_empty.push(j);
                    }
                }
            }
        }

        let empty_rows: Vec<usize> = (0..image.len())
            .filter(|i| !row_not_empty.contains(&i))
            .collect();

        let empty_cols: Vec<usize> = (0..image[0].len())
            .filter(|j| !col_not_empty.contains(&j))
            .collect();

        Self {
            image,
            empty_rows,
            empty_cols,
        }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn get_galaxies(image: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    image
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| match *c == '#' {
                    true => Some((i, j)),
                    false => None,
                })
        })
        .collect()
}

fn distance(
    g1: (usize, usize),
    g2: (usize, usize),
    factor: usize,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
) -> usize {
    let (m1, n1) = (g1.0.min(g2.0), g1.1.min(g2.1));
    let (m2, n2) = (g1.0.max(g2.0), g1.1.max(g2.1));

    let er = (m1..m2)
        .filter(|i| empty_rows.contains(&i))
        .collect::<Vec<usize>>()
        .len();

    let ec = (n1..n2)
        .filter(|j| empty_cols.contains(&j))
        .collect::<Vec<usize>>()
        .len();

    let nr = (m2 - m1) - er;
    let nc = (n2 - n1) - ec;

    nr + nc + (factor * (er + ec))
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let universe = Universe::from(input);
    let galaxies = get_galaxies(universe.image);
    let factor = 2;

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| {
            galaxies
                .iter()
                .enumerate()
                .filter(move |(j, _)| j > &i)
                .map(|(_, g2)| {
                    distance(
                        *g1,
                        *g2,
                        factor,
                        universe.empty_rows.clone(),
                        universe.empty_cols.clone(),
                    )
                })
                .collect::<Vec<usize>>()
        })
        .sum()
}

fn pt2(input: &str) -> usize {
    let universe = Universe::from(input);
    let galaxies = get_galaxies(universe.image);
    let factor = 1000000;

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| {
            galaxies
                .iter()
                .enumerate()
                .filter(move |(j, _)| j > &i)
                .map(|(_, g2)| {
                    distance(
                        *g1,
                        *g2,
                        factor,
                        universe.empty_rows.clone(),
                        universe.empty_cols.clone(),
                    )
                })
                .collect::<Vec<usize>>()
        })
        .sum()
}

pub fn day11() {
    let input = read_input("./src/day11/puzzle_input.txt");
    println!("Day 11:");
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\
";

        assert_eq!(pt1(puzzle_input), 374);
    }
}
