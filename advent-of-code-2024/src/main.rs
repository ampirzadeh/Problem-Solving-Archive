use advent_of_code_2024::Solution;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use std::{env, fs::File, io::Read, path::PathBuf};

mod day1;
mod day2;
mod day3;

fn main() {
    let day_number = env::args()
        .nth(1)
        .expect("Pass the day number as an arguement")
        .parse::<i32>()
        .expect("Pass the day as just a number");

    let path = PathBuf::from(format!("src/data/day{}.txt", day_number));

    let mut file = File::open(path).expect("Couldn't get the input file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Failed to read file content");
    let input = String::from_utf8(contents)
        .expect("File isn't valid UTF")
        .trim_end_matches("\n")
        .to_string();

    let solutions: Vec<Box<dyn Solution>> = vec![
        Box::new(Day1 {
            input: input.clone(),
        }),
        Box::new(Day2 {
            input: input.clone(),
        }),
        Box::new(Day3 {
            input: input.clone(),
        }),
    ];

    let solution = solutions
        .iter()
        .nth((day_number - 1).try_into().expect("Enter a valid day"))
        .expect("Enter a valid day");

    println!(
        "Day {day_number}: part 1: {} part 2: {}",
        solution.part1(),
        solution.part2()
    );
}
