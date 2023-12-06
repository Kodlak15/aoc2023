// -------------------------------------------------------
// Advent of Code 2023 - Day 5
// -------------------------------------------------------

use std::collections::HashMap;

use crate::read_input;

// -------------------------------------------------------
// Custom data structures
// -------------------------------------------------------

struct Almanac {
    maps: HashMap<String, Vec<(u64, u64, u64)>>,
}

impl Almanac {
    fn from(data: Vec<String>) -> Self {
        let mut maps: HashMap<String, Vec<(u64, u64, u64)>> = HashMap::new();
        for group in data.iter() {
            let mut group: Vec<String> = group.split(":\n").map(|s| s.to_string()).collect();
            let key = group.remove(0).strip_suffix(" map").unwrap().to_string();
            let group: Vec<String> = group
                .remove(0)
                .split("\n")
                .map(|line| line.to_string())
                .collect();

            let mut ranges: Vec<(u64, u64, u64)> = vec![];
            for sg in group {
                let sg: Vec<u64> = sg
                    .split(" ")
                    .filter_map(|s| match s.is_empty() {
                        true => None,
                        false => Some(s.parse().unwrap()),
                    })
                    .collect();

                if !sg.is_empty() {
                    let dest_start = sg[0];
                    let source_start = sg[1];
                    let length = sg[2];

                    ranges.push((dest_start, source_start, length));
                }
            }

            maps.insert(key, ranges);
        }

        Self { maps }
    }

    fn find_location_number(&self, seed: u64) -> u64 {
        let keys = [
            "seed-to-soil".to_string(),
            "soil-to-fertilizer".to_string(),
            "fertilizer-to-water".to_string(),
            "water-to-light".to_string(),
            "light-to-temperature".to_string(),
            "temperature-to-humidity".to_string(),
            "humidity-to-location".to_string(),
        ];

        let mut m = seed;
        for key in keys {
            for (dest_start, source_start, length) in &self.maps[&key] {
                let flag: bool = (*source_start..*source_start + *length).contains(&m);

                m = match flag {
                    true => m - source_start + dest_start,
                    false => m,
                };

                if flag == true {
                    break;
                }
            }
        }

        m
    }
}

// -------------------------------------------------------
// Main program logic
// -------------------------------------------------------

fn pt1(input: &str) -> u64 {
    let mut data: Vec<String> = input
        .split("\n\n")
        .map(|line| line.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let seeds: Vec<u64> = data
        .remove(0)
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let almanac = Almanac::from(data);

    seeds
        .iter()
        .map(|seed| almanac.find_location_number(*seed))
        .min()
        .unwrap()
}

fn pt2(input: &str) -> u64 {
    let mut data: Vec<String> = input
        .split("\n\n")
        .map(|line| line.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let mut seeds: Vec<u64> = data
        .remove(0)
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let almanac = Almanac::from(data);

    let mut location_number: u64 = u64::MAX;
    while !seeds.is_empty() {
        let mut length = seeds.pop().unwrap();
        let start = seeds.pop().unwrap();

        while length > 0 {
            let current = almanac.find_location_number(start + length);
            location_number = match current < location_number {
                true => current,
                false => location_number,
            };

            length -= 1
        }
    }

    location_number
}

pub fn day05() {
    let input = read_input("./src/day05/puzzle_input.txt");
    println!("Day 5:");
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
";

        assert_eq!(pt1(puzzle_input), 35);
    }

    #[test]
    fn test_pt2() {
        let puzzle_input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
";

        assert_eq!(pt2(puzzle_input), 46);
    }
}
