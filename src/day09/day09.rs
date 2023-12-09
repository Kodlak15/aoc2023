// -------------------------------------------------------
// Advent of Code 2023 - Day 9
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut history: Vec<Vec<i32>> = vec![line
                .split(" ")
                .map(|c| { c.parse::<i32>() }.expect("Unable to parse character!"))
                .collect()];

            while history[history.len() - 1].iter().any(|x| *x != 0) {
                let last = &history[history.len() - 1];

                let next: Vec<i32> = last
                    .iter()
                    .enumerate()
                    .filter_map(|(i, _)| match i < last.len() - 1 {
                        true => Some(last[i + 1] - last[i]),
                        false => None,
                    })
                    .collect();

                history.push(next);
            }

            let mut values: Vec<i32> = Vec::new();
            values.push(0);

            for (i, _) in history.iter().enumerate().rev().skip(1) {
                let seq = &history[i];
                let m = seq.len();

                values.push(seq[m - 1] + values[values.len() - 1])
            }

            values[values.len() - 1]
        })
        .sum()
}

fn pt2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut history: Vec<Vec<i32>> = vec![line
                .split(" ")
                .map(|c| { c.parse::<i32>() }.expect("Unable to parse character!"))
                .collect()];

            while history[history.len() - 1].iter().any(|x| *x != 0) {
                let last = &history[history.len() - 1];

                let next: Vec<i32> = last
                    .iter()
                    .enumerate()
                    .filter_map(|(i, _)| match i < last.len() - 1 {
                        true => Some(last[i + 1] - last[i]),
                        false => None,
                    })
                    .collect();

                history.push(next);
            }

            // New logic goes here

            let mut values: Vec<i32> = Vec::new();
            values.push(0);

            for (i, _) in history.iter().enumerate().rev().skip(1) {
                let seq = &history[i];
                let m = seq.len();

                values.push(seq[m - 1] + values[values.len() - 1])
            }

            values[values.len() - 1]
        })
        .sum()
}

pub fn day09() {
    let input = read_input("./src/day09/puzzle_input.txt");
    println!("Day 9:");
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
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\
";

        assert_eq!(pt1(puzzle_input), 114);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\
";

        assert_eq!(pt2(puzzle_input), 2);
    }
}
