// -------------------------------------------------------
// Advent of Code 2023 - Day 12
// -------------------------------------------------------

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

struct Records {
    groups: Vec<Vec<String>>,
    sizes: Vec<Vec<usize>>,
}

impl Records {
    fn from(input: &str) -> Self {
        let mut groups: Vec<Vec<String>> = Vec::new();
        let mut sizes: Vec<Vec<usize>> = Vec::new();

        let data: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .collect();

        for row in &data {
            groups.push(
                row[0]
                    .split(".")
                    .filter_map(|s| match !s.is_empty() {
                        true => Some(s.to_string()),
                        false => None,
                    })
                    .collect::<Vec<String>>(),
            );

            sizes.push(
                row[1]
                    .split(",")
                    .map(|s| s.parse::<usize>().expect("Could not parse size!"))
                    .collect(),
            );
        }

        println!("Groups: {:?}", groups);
        println!("Sizes: {:?}", sizes);

        Self { groups, sizes }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let records = Records::from(input);

    0
}

fn pt2(input: &str) -> usize {
    0
}

pub fn day12() {
    let input = read_input("./src/day12/puzzle_input.txt");
    println!("Day 12:");
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
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1\
";

        assert_eq!(pt1(puzzle_input), 21);
    }
}
