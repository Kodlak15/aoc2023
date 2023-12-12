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
}

impl Universe {
    fn from(input: &str, expansion_factor: u32) -> Self {
        let mut image: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        image = expand(&mut image, expansion_factor);
        println!("Image expanded...");
        image = transpose(&mut image);
        println!("Image transposed...");
        image = expand(&mut image, expansion_factor);
        println!("Image expanded...");
        image = transpose(&mut image);
        println!("Image transposed...");

        println!("Expanded image rows: {:?}", image.len());
        println!("Expanded image cols: {:?}", image[0].len());

        Self { image }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn expand(image: &mut Vec<Vec<char>>, factor: u32) -> Vec<Vec<char>> {
    let mut expanded_image: Vec<Vec<char>> = Vec::new();

    while !image.is_empty() {
        let row = image.pop().expect("Could not unpack image row!");

        if row.iter().all(|c| *c == '.') {
            for _ in 0..factor - 1 {
                expanded_image.push(row.clone());
            }
        }

        expanded_image.push(row);
    }

    expanded_image
}

// Out of memory error due to this implementation
fn transpose(image: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed_image: Vec<Vec<char>> = Vec::new();

    let col = image.len() - 1;

    for i in 0..image[0].len() {
        let transposed_row: Vec<char> = image.iter().rev().map(|row| row[i]).collect();
        transposed_image.push(transposed_row.clone());
    }

    transposed_image
}

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

fn distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    let (m1, n1) = g1;
    let (m2, n2) = g2;

    (m1.max(m2) - m1.min(m2)) + (n1.max(n2) - n1.min(n2))
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let universe = Universe::from(input, 2);
    let galaxies = get_galaxies(universe.image);

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| {
            galaxies
                .iter()
                .enumerate()
                .filter(move |(j, _)| j > &i)
                .map(|(_, g2)| distance(*g1, *g2))
                .collect::<Vec<usize>>()
        })
        .sum()
}

fn pt2(input: &str) -> usize {
    let universe = Universe::from(input, 1000000);
    let galaxies = get_galaxies(universe.image);

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g1)| {
            galaxies
                .iter()
                .enumerate()
                .filter(move |(j, _)| j > &i)
                .map(|(_, g2)| distance(*g1, *g2))
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

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 6);
    //     }
}
