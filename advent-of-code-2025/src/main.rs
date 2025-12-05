use advent_of_code_2025::Solution;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn file(day_number: usize) -> String {
    let path = PathBuf::from(format!("src/data/day{}.txt", day_number));
    let input: String = fs::read_to_string(path)
        .expect("Error reading file content")
        .trim()
        .to_string();
    return input;
}

fn main() {
    let solutions: [Box<dyn Solution>; 5] = [
        Box::new(Day1 { input: file(1) }),
        Box::new(Day2 { input: file(2) }),
        Box::new(Day3 { input: file(3) }),
        Box::new(Day4 { input: file(4) }),
        Box::new(Day5 { input: file(5) }),
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
