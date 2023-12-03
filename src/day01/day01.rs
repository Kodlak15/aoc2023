// Advent of Code - Day 1:
// Updated solution after drawing some inspiration from solutions shared on reddit

use crate::read_input;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

// For each line of the input, find the first and last digits that occur and generate a two digit
// number from them. If only one digit is found, repeat the first digit. For example, if only a 7
// is found, the resulting two digit number is 77. Finally, return the sum of all two digit numbers
// found.
fn pt1(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let a = digits.next()?;
            let b = digits.rev().next().unwrap_or(a);
            let x: u32 = (a * 10) + b;
            Some(x)
        })
        .sum();

    sum
}

// For each line of the input, begin by replacing any string representation of a digit with the digit
// itself surrouded by the first and last characters of that string. For example, if the string
// 'three' is encoutered, replace it with 't3e'. The reason it is necessary to keep the first and
// last digits of the string is to handle edge cases where the first/last letter of one string
// is a part of another string representation of a digit, as is the case in 'threeight' or
// 'fiveighthreeeightwo'. Finally, repeat the procedure from part 1 to get the sum of all two digit
// numbers found.
fn pt2(input: &str) -> u32 {
    let sum: u32 = input
        .lines()
        .map(|line| line.to_string())
        .filter_map(|line| {
            NUMBERS
                .into_iter()
                .enumerate()
                .try_fold(line, |line, (n, s)| {
                    let first_char = s.chars().next()?;
                    let last_char = s.chars().next_back()?;
                    let target = format!("{first_char}{n}{last_char}");
                    Some(line.replace(s, &target))
                })
        })
        .filter_map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let a = digits.next()?;
            let b = digits.next_back().unwrap_or(a);
            let x = (a * 10) + b;
            Some(x)
        })
        .sum();

    sum
}

pub fn day01() {
    let input = read_input("./src/day02/puzzle_input.txt");
    println!("Day 1:");
    println!("Sum of calibration values (part 1): {}", pt1(&input));
    println!("Sum of calibration values (part 2): {}", pt2(&input));
    println!("-------------------------------------------------------")
}

#[test]
fn test_pt1() {
    let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
    .to_string();
    let sum = pt1(&input);
    assert_eq!(sum, 142)
}

#[test]
fn test_pt2() {
    let input = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
    .to_string();
    let sum = pt2(&input);
    assert_eq!(sum, 281)
}
