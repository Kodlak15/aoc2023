use aoc2023::{
    day01::day01::day01, day02::day02::day02, day03::day03::day03, day04::day04::day04,
    day05::day05::day05, day06::day06::day06, day07::day07::day07,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    day: Option<i32>,
}

fn main() {
    let args = Args::parse();

    match args.day {
        Some(1) => day01(),
        Some(2) => day02(),
        Some(3) => day03(),
        Some(4) => day04(),
        Some(5) => day05(),
        Some(6) => day06(),
        Some(7) => day07(),
        Some(0) => {
            day01();
            day02();
            day03();
            day04();
            day05();
            day06();
            day07();
        }
        _ => (),
    }
}
