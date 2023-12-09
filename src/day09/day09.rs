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

fn pt1(input: &str) -> usize {
    let sequences: usize = input
        .lines()
        .map(|line| {
            let mut history: Vec<Vec<usize>> = vec![line
                .split(" ")
                .map(|c| c.parse::<usize>().expect("Unable to parse character!"))
                .collect()];

            while history[history.len() - 1].iter().any(|x| *x != 0) {
                let last = &history[history.len() - 1];

                let next: Vec<usize> = last
                    .iter()
                    .enumerate()
                    .filter_map(|(i, _)| match i < last.len() - 1 {
                        true => Some(last[i + 1] - last[i]),
                        false => None,
                    })
                    .collect();

                history.push(next);
            }

            for (i, seq) in history.iter().enumerate().rev() {
                let n = history[i].len();
                let v = history[i][n] + history[i - 1][n];

                // Cannot edit history as we are iterating over it (this way at least)
                history[i].push(v);
            }

            0
        })
        .sum();

    println!("Sequences {:?}", sequences);

    0
}

// fn pt2(input: &str) -> usize {
//     0
// }

pub fn day09() {
    let input = read_input("./src/day09/puzzle_input.txt");
    println!("Day 8:");
    println!("Part 1: {}", pt1(&input));
    // println!("Part 2: {}", pt2(&input));
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

    //     #[test]
    //     fn test_pt2() {
    //         let puzzle_input = "\
    // ";
    //
    //         assert_eq!(pt2(puzzle_input), 6);
    //     }
}
