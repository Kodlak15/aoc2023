// -------------------------------------------------------
// Advent of Code 2023 - Day 7
// -------------------------------------------------------

use std::collections::HashMap;

use crate::read_input;

// -------------------------------------------------------
// Global Variables
// -------------------------------------------------------

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl Hand {
    fn from(cards: String) -> Self {
        let counts: Vec<u32> = cards
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .iter()
            .map(|(_, count)| count.to_string().parse::<u32>().unwrap())
            .collect();

        match counts.len() {
            1 => Self::FiveOfAKind(cards),
            2 => match counts.contains(&4) {
                true => Self::FourOfAKind(cards),
                false => Self::FullHouse(cards),
            },
            3 => match counts.contains(&3) {
                true => Self::ThreeOfAKind(cards),
                false => Self::TwoPair(cards),
            },
            4 => Self::OnePair(cards),
            5 => Self::HighCard(cards),
            _ => panic!("Invalid hand!"),
        }
    }

    fn cards(&self) -> &str {
        match &self {
            Self::FiveOfAKind(cards) => cards,
            Self::FourOfAKind(cards) => cards,
            Self::FullHouse(cards) => cards,
            Self::ThreeOfAKind(cards) => cards,
            Self::TwoPair(cards) => cards,
            Self::OnePair(cards) => cards,
            Self::HighCard(cards) => cards,
            _ => panic!("Invalid hand!"),
        }
    }
}

#[derive(Debug)]
struct HandSet {
    hands: Vec<(Hand, u32, u32)>,
}

impl HandSet {
    fn from(input: &str) -> Self {
        let mut hands: Vec<(Hand, u32, u32)> = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .map(|pair| {
                let cards = Hand::from(pair[0].to_string());
                let value = hand_value(cards.cards());
                let bid = pair[1].parse::<u32>().unwrap();

                (cards, value, bid)
            })
            .collect::<Vec<(Hand, u32, u32)>>();

        hands.sort_by(|h1, h2| {
            h1.0.partial_cmp(&h2.0)
                .unwrap()
                .then_with(|| h2.1.cmp(&h1.1))
        });

        println!("Hands: {:?}", hands);

        Self { hands }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn card_value(card: char) -> usize {
    CARDS.iter().position(|c| *c == card).unwrap()
}

fn hand_value(cards: &str) -> u32 {
    cards
        .chars()
        .map(|card| card_value(card))
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .concat()
        .parse::<u32>()
        .unwrap()
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

#[allow(dead_code)]
fn pt1(input: &str) -> u32 {
    let hand_set = HandSet::from(input);

    let hmap: HashMap<&str, Vec<String>> =
        hand_set.hands.iter().fold(HashMap::new(), |mut acc, hand| {
            match &hand.0 {
                Hand::FiveOfAKind(cards) => {
                    acc.entry("FiveOfAKind")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::FourOfAKind(cards) => {
                    acc.entry("FourOfAKind")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::FullHouse(cards) => {
                    acc.entry("FullHouse")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::ThreeOfAKind(cards) => {
                    acc.entry("ThreeOfAKind")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::TwoPair(cards) => {
                    acc.entry("TwoPair")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::OnePair(cards) => {
                    acc.entry("OnePair")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                Hand::HighCard(cards) => {
                    acc.entry("HighCard")
                        .or_insert(Vec::new())
                        .push(cards.clone());
                }
                _ => panic!("Invalid hand!"),
            };

            acc
        });

    // for key in hmap.keys() {
    //     println!("{}", key)
    // }

    0
}

#[allow(dead_code)]
fn pt2(_input: &str) -> u32 {
    0
}

pub fn day07() {
    let input = read_input("./src/day07/puzzle_input.txt");
    println!("Day 7:");
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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483\
";

        assert_eq!(pt1(puzzle_input), 6440);
    }

    #[test]
    fn test_pt2() {
        let _puzzle_input = "\
\
";

        // assert_eq!(pt2(puzzle_input), 71503);
    }
}
