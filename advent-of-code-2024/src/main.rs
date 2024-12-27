use advent_of_code_2024::Solution;
use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day2::Day2;
use day22::Day22;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn file(day_number: usize) -> String {
    let path = PathBuf::from(format!("src/data/day{}.txt", day_number));
    let input: String = fs::read_to_string(path)
        .expect("Error reading file content")
        .trim()
        .to_string();
    return input;
}

fn main() {
    let solutions: [Box<dyn Solution>; 20] = [
        Box::new(Day1 { input: file(1) }),
        Box::new(Day2 { input: file(2) }),
        Box::new(Day3 { input: file(3) }),
        Box::new(Day4 { input: file(4) }),
        Box::new(Day5 { input: file(5) }),
        Box::new(Day6 { input: file(6) }),
        Box::new(Day7 { input: file(7) }),
        Box::new(Day8 { input: file(8) }),
        Box::new(Day9 { input: file(9) }),
        Box::new(Day10 { input: file(10) }),
        Box::new(Day11 { input: file(11) }),
        Box::new(Day12 { input: file(12) }),
        Box::new(Day13 { input: file(13) }),
        Box::new(Day14 { input: file(14) }),
        Box::new(Day15 { input: file(15) }),
        Box::new(Day16 { input: file(16) }),
        Box::new(Day17 { input: file(17) }),
        Box::new(Day18 { input: file(18) }),
        Box::new(Day19 { input: file(19) }),
        Box::new(Day22 { input: file(22) }),
    ];

    match env::args()
        .nth(1)
        .and_then(|d| Some(d.parse::<usize>().expect("Pass the day as just a number")))
    {
        Some(day_number) => {
            let solution = solutions.get(day_number - 1).expect("Enter a valid day");
            println!(
                "Day {}: part 1: {} part 2: {}",
                day_number,
                solution.part1(),
                solution.part2()
            );
        }
        None => {
            for (day_number, solution) in solutions.iter().enumerate() {
                println!(
                    "Day {}: part 1: {} part 2: {}",
                    day_number + 1,
                    solution.part1(),
                    solution.part2()
                );
            }
        }
    }
}
