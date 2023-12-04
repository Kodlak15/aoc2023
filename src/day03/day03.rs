// NOTE: I am a moron and was using input from day 2 all day yesterday

use regex::Regex;

use crate::read_input;

#[allow(dead_code)]
struct Schematic {
    grid: Vec<String>,
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl Schematic {
    fn from(input: &str) -> Self {
        let grid: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let rows = grid.len();
        let cols = grid[0].len();

        Self { grid, rows, cols }
    }

    fn validate_coords(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }
}

#[allow(dead_code)]
struct Engine {
    schematic: Schematic,
}

#[allow(dead_code)]
impl Engine {
    fn from(input: &str) -> Self {
        let schematic = Schematic::from(input);

        Engine { schematic }
    }
}

#[allow(dead_code)]
fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

#[allow(dead_code)]
fn pt1(input: &str) -> u32 {
    let engine = Engine::from(input);
    let re = Regex::new(r"\d+").unwrap();

    let part_nums: Vec<u32> = engine
        .schematic
        .grid
        .iter()
        .enumerate()
        .flat_map(|(m, line)| {
            re.find_iter(line).flat_map(|rematch| {
                println!("Match {:?}", rematch);
                Some(rematch.as_str().parse::<u32>().unwrap())
            })

            // Some(0)
        })
        .collect();

    part_nums.iter().sum()
}

#[allow(dead_code)]
fn pt2(_input: &str) -> u32 {
    0
}

pub fn day03() {
    let input = read_input("./src/day03/puzzle_input.txt");
    println!("Day 3:");
    // println!("Part 1: {}", pt1(&input));
    // println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

// #[test]
// fn test_pt1() {
//     let input = "\
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// ";
//
//     assert_eq!(pt1(input), 4361);
// }

// #[test]
// fn test_pt2() {
//     unimplemented!()
// }
