use std::collections::HashMap;

use regex::Regex;

use crate::read_input;

struct Scratchcards {
    cards: Vec<String>,
}

#[allow(dead_code)]
impl Scratchcards {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"Card\s+\d+:").unwrap();
        let cards: Vec<String> = input
            .lines()
            .map(|line| {
                re.find_iter(line)
                    .map(|m| line.to_string().replace(m.as_str(), ""))
                    .collect()
            })
            .collect();

        Scratchcards { cards }
    }

    fn get(&self, id: usize) -> &String {
        &self.cards[id]
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn matches(&self, id: usize) -> u32 {
        let re = Regex::new(r"\d+").unwrap();
        let card: Vec<&str> = self.get(id).split(" | ").collect();
        let targets: Vec<&str> = re.find_iter(card[0]).map(|m| m.as_str()).collect();

        let sum: u32 = re
            .find_iter(card[1])
            .filter_map(|m| match targets.contains(&m.as_str()) {
                true => Some(1),
                false => None,
            })
            .sum();

        sum
    }

    fn copies(&self, id: usize) -> u32 {
        0
    }
}

fn pt1(input: &str) -> u32 {
    let scratchcards = Scratchcards::from(input);

    let base: u32 = 2;
    let mut points: u32 = 0;
    for id in 0..scratchcards.len() {
        let matches = scratchcards.matches(id);
        points += match matches > 0 {
            true => base.pow(matches - 1),
            false => 0,
        };
    }

    points
}

// Number is too high
fn pt2(input: &str) -> u32 {
    let scratchcards = Scratchcards::from(input);
    let mut n: usize = scratchcards.len() - 1;

    // { N : (Copies, Value) }
    let mut hmap: HashMap<usize, (usize, usize)> = HashMap::new();

    while n > 0 {
        let num_matches: usize = scratchcards.matches(n).try_into().unwrap();
        let mut stack: Vec<usize> = (n + 1..scratchcards.len().min(n + num_matches + 1)).collect();

        hmap.insert(n, (1, 1));
        while stack.len() > 0 {
            let id = stack.pop().unwrap();
            let (mut c1, v1) = hmap[&id];
            let (c2, v2) = hmap[&n];
            c1 += 1;
            hmap.insert(id, (c1, v1));
            hmap.insert(n, (c2, v2 + (c1 * v1)));
        }

        n -= 1;
    }

    println!("HashMap: {:?}", hmap);
    hmap[&1].1.try_into().unwrap()
}

pub fn day04() {
    let input = read_input("./src/day04/puzzle_input.txt");
    println!("Day 4:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
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
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";

        assert_eq!(pt1(puzzle_input), 13);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
    ";

        assert_eq!(pt2(puzzle_input), 30);
    }
}
