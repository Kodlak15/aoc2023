use aoc2023::{
    day01::day01::day01, day02::day02::day02, day03::day03::day03, day04::day04::day04,
    day05::day05::day05, day06::day06::day06, day07::day07::day07, day08::day08::day08,
    day09::day09::day09, day10::day10::day10, day11::day11::day11, day12::day12::day12,
    day13::day13::day13, day14::day14::day14, day15::day15::day15, day16::day16::day16,
    day17::day17::day17, day18::day18::day18,
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
        Some(8) => day08(),
        Some(9) => day09(),
        Some(10) => day10(),
        Some(11) => day11(),
        Some(12) => day12(),
        Some(13) => day13(),
        Some(14) => day14(),
        Some(15) => day15(),
        Some(16) => day16(),
        Some(17) => day17(),
        Some(18) => day18(),
        Some(0) => {
            day01();
            day02();
            day03();
            day04();
            day05();
            day06();
            day07();
            day08();
            day09();
            day10();
            day11();
            day12();
            day13();
            day14();
            day15();
            day16();
            day17();
            day18();
        }
        _ => (),
    }
}
