// -------------------------------------------------------
// Advent of Code 2023 - Day 7
// -------------------------------------------------------

use std::collections::HashMap;

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn from(card: char) -> Self {
        match card {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'W' => Self::Joker,
            _ => panic!("Invalid card!"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Hand {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl Hand {
    fn from(cards: String) -> Self {
        let maxfreqcard = cards
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                if c != 'W' {
                    *acc.entry(c).or_insert(0) += 1;
                }

                acc
            })
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(c, _)| c);

        let maxfreqcard = match maxfreqcard {
            Some(c) => c,
            None => 'W',
        };

        let counts: Vec<u32> = cards
            .chars()
            .fold(HashMap::new(), |mut acc, c| {
                match c == 'W' {
                    true => *acc.entry(maxfreqcard).or_insert(0) += 1,
                    false => *acc.entry(c).or_insert(0) += 1,
                };

                acc
            })
            .iter()
            .map(|(_, count)| count.to_string().parse::<u32>().unwrap())
            .collect();

        let mut hand: Vec<Card> = cards.chars().map(|c| Card::from(c)).collect();

        match counts.len() {
            1 => Self::FiveOfAKind(hand),
            2 => match counts.contains(&4) {
                true => Self::FourOfAKind(hand),
                false => Self::FullHouse(hand),
            },
            3 => match counts.contains(&3) {
                true => Self::ThreeOfAKind(hand),
                false => Self::TwoPair(hand),
            },
            4 => Self::OnePair(hand),
            5 => Self::HighCard(hand),
            _ => panic!("Invalid hand!"),
        }
    }

    fn cards(&self) -> &Vec<Card> {
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
    hands: Vec<(Hand, usize)>,
}

impl HandSet {
    fn from(input: &str) -> Self {
        let mut hands: Vec<(Hand, usize)> = input
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .map(|pair| {
                let cards = Hand::from(pair[0].to_string());
                let bid = pair[1].parse::<usize>().unwrap();

                (cards, bid)
            })
            .collect::<Vec<(Hand, usize)>>();

        hands.sort_by(|h1, h2| h2.0.partial_cmp(&h1.0).unwrap());

        Self { hands }
    }
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

#[allow(dead_code)]
fn pt1(input: &str) -> usize {
    let hands = HandSet::from(input);

    hands
        .hands
        .iter()
        .enumerate()
        .map(|(r, (_, bid))| (r + 1) * bid)
        .sum()
}

#[allow(dead_code)]
fn pt2(input: &str) -> usize {
    let hands = HandSet::from(input.replace("J", "W").as_str());

    hands
        .hands
        .iter()
        .enumerate()
        .map(|(r, (_, bid))| (r + 1) * bid)
        .sum()
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
        let puzzle_input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483\
";

        assert_eq!(pt2(puzzle_input), 5905);
    }
}
