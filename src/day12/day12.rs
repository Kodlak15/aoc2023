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

fn sliding_window(group: String, size: usize) -> usize {
    let n = group.len();
    let mut i = 0;
    let mut j = size;
    let mut arrangements = 0;

    while j < n {
        if group[i..j].chars().all(|c| c == '#') {
            return 1;
        }

        i += 1;
        j += 1;
        arrangements += 1;
    }

    arrangements
}

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> usize {
    let records = Records::from(input);

    let _ = records
        .groups
        .iter()
        .zip(records.sizes.iter())
        .map(|(groups, sizes)| match groups.len() == sizes.len() {
            true => (0..groups.len())
                .map(|i| sliding_window(groups[i].clone(), sizes[i]))
                .sum(),
            false => 0,
        });

    0
}

fn pt2(_input: &str) -> usize {
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
