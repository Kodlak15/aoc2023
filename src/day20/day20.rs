// -------------------------------------------------------
// Advent of Code 2023 - Day 20
// -------------------------------------------------------

use std::collections::{HashMap, VecDeque};

use crate::read_input;

// -------------------------------------------------------
// Custom Data Structures
// -------------------------------------------------------

#[derive(Debug)]
enum Pulse {
    Low,
    High,
    Nil,
}

// TODO: this is incomplete
#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    // Need to be able to identify pulses from specific inputs
    Conjunction(Vec<Pulse>),
    Broadcast,
}

impl Module {
    // TODO: this is incomplete
    fn from(c: char) -> Self {
        match c {
            '%' => Self::FlipFlop(false),
            // Need to be able to identify pulses from specific inputs
            '&' => Self::Conjunction(vec![]),
            'b' => Self::Broadcast,
            _ => panic!("Invalid module identifier!"),
        }
    }

    // TODO: this is incomplete
    fn process(&mut self, pulse: Pulse) -> Pulse {
        match self {
            Self::FlipFlop(flag) => match pulse {
                Pulse::High => Pulse::Nil,
                Pulse::Low => match flag {
                    true => {
                        *self = Self::FlipFlop(false);
                        Pulse::Low
                    }
                    false => {
                        *self = Self::FlipFlop(true);
                        Pulse::High
                    }
                },
                Pulse::Nil => Pulse::Nil,
            },
            // Need to be able to identify pulses from specific inputs
            Self::Conjunction(pulses) => match pulses.iter().all(|p| match p {
                Pulse::High => true,
                Pulse::Low => false,
                // TODO: not sure if this is correct
                Pulse::Nil => true,
            }) {
                true => Pulse::Low,
                false => Pulse::High,
            },
            Self::Broadcast => Pulse::Low,
        }
    }
}

struct Config<'a> {
    modules: HashMap<&'a str, (Module, Vec<&'a str>)>,
}

impl<'a> Config<'a> {
    fn from(input: &'a str) -> Self {
        let modules: HashMap<&str, (Module, Vec<&str>)> = input
            .lines()
            .map(|line| {
                let line: Vec<&str> = line.split(" -> ").collect();

                let module = match line[0].chars().nth(0) {
                    Some(c) => Module::from(c),
                    None => panic!("No module identifier found!"),
                };

                let name = match module {
                    Module::FlipFlop(_) => &line[0][1..],
                    Module::Conjunction(_) => &line[0][1..],
                    Module::Broadcast => "broadcast",
                };

                let destinations: Vec<&str> = line[1].split(", ").collect();

                (name, (module, destinations))
            })
            .collect();

        Self { modules }
    }

    fn next_state(&mut self) {
        let (broadcast, destinations) = &self.modules["broadcast"];

        let queue: VecDeque<&str> = destinations.iter().map(|s| *s).collect();

        while !queue.is_empty() {}
    }
}

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

// -------------------------------------------------------
// Main Program Logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let config = Config::from(input);

    let n = 1000;

    0
}

fn pt2(_input: &str) -> u32 {
    0
}

pub fn day20() {
    let input = read_input("./src/day20/puzzle_input.txt");
    println!("Day 20:");
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
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a\
";

        assert_eq!(pt1(puzzle_input), 32000000);
    }

    #[test]
    fn test2_pt1() {
        let puzzle_input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output\
";

        assert_eq!(pt1(puzzle_input), 11687500);
    }

    #[test]
    fn test_pt2() {
        // let puzzle_input = "";

        // assert_eq!(pt2(puzzle_input), 145);
    }
}
