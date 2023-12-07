// -------------------------------------------------------
// Advent of Code 2023 - Day 6
// -------------------------------------------------------

use regex::Regex;

use crate::read_input;

// -------------------------------------------------------
// Helper Functions
// -------------------------------------------------------

fn distance<T>(input_time: T, race_length: T) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    let speed = input_time;
    let time = race_length - speed;

    speed * time
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .into_iter()
        .collect();

    let races: Vec<(u32, u32)> = data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(&a, &b)| (a, b))
        .collect();

    races
        .iter()
        .map(|(race_length, race_record)| {
            let breakers: Vec<u32> = (0..*race_length)
                .into_iter()
                .filter_map(|input_time| {
                    let d = distance(input_time, *race_length);

                    match d > *race_record {
                        true => Some(1),
                        false => None,
                    }
                })
                .collect();

            <usize as TryInto<u32>>::try_into(breakers.len()).unwrap()
        })
        .product()
}

fn pt2(input: &str) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<String>>()
        })
        .map(|v| v.concat().parse::<u64>().unwrap())
        .collect();

    let race_length = data[0];
    let race_record = data[1];

    (0..race_length)
        .filter_map(|input_time| {
            let d = distance(input_time, race_length);

            match d > race_record {
                true => Some(1),
                false => None,
            }
        })
        .sum()
}

pub fn day06() {
    let input = read_input("./src/day06/puzzle_input.txt");
    println!("Day 6:");
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
Time:      7  15   30
Distance:  9  40  200\
";

        assert_eq!(pt1(puzzle_input), 288);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
Time:      7  15   30
Distance:  9  40  200
\
";

        assert_eq!(pt2(puzzle_input), 71503);
    }
}
