pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

use std::fs;

pub fn read_input(fname: &str) -> String {
    fs::read_to_string(fname).expect("Failed to read puzzle input...")
}
