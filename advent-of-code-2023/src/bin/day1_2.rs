use log;
use setupper;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn match_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0,
    }
}

// Tuple (start_index_of_match, matched_string)
type MatchedDigit<'a> = (usize, &'a str);

fn main() {
    setupper::logger();

    let p = Path::new("src/data/day1.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);

    let mut sum = 0;
    let mut first_digit: MatchedDigit;
    let mut last_digit: MatchedDigit;
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    for (line_idx, line) in reader.lines().enumerate() {
        first_digit = (usize::MAX, "");
        last_digit = (0, "");

        let line = line.unwrap();

        for pattern in patterns {
            match line.find(pattern) {
                Some(idx) => {
                    if idx <= first_digit.0 {
                        first_digit = (idx, &pattern)
                    }
                }
                None => (),
            }
            match line.rfind(pattern) {
                Some(idx) => {
                    if idx >= last_digit.0 {
                        last_digit = (idx, &pattern);
                    }
                }
                None => (),
            }
        }

        log::debug!(
            "Line {line_idx}: first_digit {} last_digit {}",
            match_digit(first_digit.1),
            match_digit(last_digit.1)
        );
        sum += match_digit(first_digit.1) * 10 + match_digit(last_digit.1);
    }

    log::info!("{}", sum);
}
