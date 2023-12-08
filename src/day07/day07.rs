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
    hands: Vec<(Hand, u32)>,
}

impl HandSet {
    fn from(input: &str) -> Self {
        let hands: Vec<(Hand, u32)> = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .map(|pair| {
                (
                    Hand::from(pair[0].to_string()),
                    pair[1].parse::<u32>().unwrap(),
                )
            })
            .collect();

        Self { hands }
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// fn compare_cards<'a>(hand1: &'a Hand, hand2: &'a Hand) -> Option<&'a Hand> {
//     let cards1 = hand1.cards();
//     let cards2 = hand2.cards();
//
//     cards1
//         .chars()
//         .zip(cards2.chars())
//         .map(|(c1, c2)| {
//             (
//                 CARDS.iter().position(|c| *c == c1),
//                 CARDS.iter().position(|c| *c == c2),
//             )
//         })
//         .take_while(|(x1, x2)| x1 != x2)
//         .last()
//         .map(|(x1, x2)| match x2 > x1 {
//             true => hand2,
//             false => hand1,
//         })
// }
//
// fn compare_hands<'a>(hand1: &'a Hand, hand2: &'a Hand) -> &'a Hand {
//     match hand1 == hand2 {
//         true => match compare_cards(hand1, hand2) {
//             Some(hand) => hand,
//             None => hand1,
//         },
//         false => match hand1 > hand2 {
//             true => hand1,
//             false => hand2,
//         },
//     }
// }

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

#[allow(dead_code)]
fn pt1(input: &str) -> u32 {
    let hand_set = HandSet::from(input);

    let mut hmap: HashMap<&str, Vec<String>> = HashMap::new();
    hmap.insert("FiveOfAKind", vec![]);
    hmap.insert("FourOfAKind", vec![]);
    hmap.insert("FullHouse", vec![]);
    hmap.insert("ThreeOfAKind", vec![]);
    hmap.insert("TwoPair", vec![]);
    hmap.insert("OnePair", vec![]);
    hmap.insert("HighCard", vec![]);

    // hand_set.hands.iter().fold(
    //     HashMap::new(),
    //     |mut acc: HashMap<&str, Vec<String>>, hand| {
    //         println!("{:?}", acc);
    //
    //         match &hand.0 {
    //             Hand::OnePair(cards) => match acc.get("OnePair") {
    //                 Some(v) => {
    //                     v.push(cards.to_string());
    //                 }
    //                 None => {
    //                     let mut v: Vec<String> = Vec::new();
    //                     v.push(cards.to_string());
    //                     acc.insert("OnePair", v);
    //                 }
    //             },
    //             _ => panic!("Invalid hand!"),
    //         };
    //
    //         acc
    //     },
    // );

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
