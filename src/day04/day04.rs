use regex::Regex;

use crate::read_input;

#[allow(dead_code)]
fn pt1(input: &str) -> u32 {
    let re = Regex::new(r"Card\s+\d+:").unwrap();
    let cards: Vec<String> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| line.to_string().replace(m.as_str(), ""))
                .collect()
        })
        .collect();

    let re = Regex::new(r"\d+").unwrap();
    let sum: u32 = cards
        .into_iter()
        .filter_map(|card| {
            let card: Vec<&str> = card.split(" | ").collect();
            let targets: Vec<&str> = re.find_iter(card[0]).map(|m| m.as_str()).collect();

            let winners: Vec<&str> = re
                .find_iter(card[1])
                .filter_map(|m| match targets.contains(&m.as_str()) {
                    true => Some(m.as_str()),
                    false => None,
                })
                .collect();

            let base: u32 = 2;
            let n: u32 = winners.len().try_into().unwrap();
            let score: u32 = match n > 0 {
                true => base.pow(n - 1),
                false => 0,
            };

            Some(score)
        })
        .sum();

    sum
}

#[allow(dead_code)]
fn pt2() {}

pub fn day04() {
    let input = read_input("./src/day04/puzzle_input.txt");
    println!("Day 4:");
    println!("Part 1: {}", pt1(&input));
    // println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let puzzle_input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        assert_eq!(pt1(puzzle_input), 13);
    }

    // #[test]
    // fn test_pt2() {
    //     unimplemented!();
    // }
}
