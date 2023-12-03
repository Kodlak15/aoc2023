// Advent of Code 2023 - Day 2:

use crate::read_input;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn pt1(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let game = line.split(": ").collect::<Vec<&str>>();
            let mut id: u32 = game[0].split("Game ").collect::<String>().parse().unwrap();
            let sets = game[1].split("; ").collect::<Vec<&str>>();

            for set in &sets {
                let set: Vec<&str> = set.split(", ").collect();
                for item in set {
                    let cube = item.split(" ").collect::<Vec<&str>>();
                    let color = cube[1];
                    let count: u32 = cube[0].parse().unwrap();
                    id = match color {
                        "red" => {
                            if count > RED {
                                0
                            } else {
                                id
                            }
                        }
                        "green" => {
                            if count > GREEN {
                                0
                            } else {
                                id
                            }
                        }
                        "blue" => {
                            if count > BLUE {
                                0
                            } else {
                                id
                            }
                        }
                        _ => id,
                    };
                }
            }

            Some(id)
        })
        .sum();

    sum
}

fn pt2(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let mut red = 1;
            let mut green = 1;
            let mut blue = 1;

            let game = line.split(": ").collect::<Vec<&str>>();
            let sets = game[1].split("; ").collect::<Vec<&str>>();

            for set in &sets {
                let set: Vec<&str> = set.split(", ").collect();
                for item in set {
                    let cube = item.split(" ").collect::<Vec<&str>>();
                    let color = cube[1];
                    let count: u32 = cube[0].parse().unwrap();

                    if color == "red" {
                        red = match count > red {
                            true => count,
                            false => red,
                        }
                    } else if color == "green" {
                        green = match count > green {
                            true => count,
                            false => green,
                        }
                    } else if color == "blue" {
                        blue = match count > blue {
                            true => count,
                            false => blue,
                        }
                    } else {
                        // Do nothing
                        ();
                    }
                }
            }

            Some(red * green * blue)
        })
        .sum();

    sum
}

pub fn day02() {
    let input = read_input("./src/day02/puzzle_input.txt");
    println!("Day 2:");
    println!("Part 1: {}", pt1(&input));
    println!("Part 2: {}", pt2(&input));
    println!("-------------------------------------------------------")
}

#[test]
fn test_pt1() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    assert_eq!(pt1(input), 8);
}

#[test]
fn test_pt2() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    assert_eq!(pt2(input), 2286);
}
