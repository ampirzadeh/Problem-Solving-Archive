use log;
use setupper;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    setupper::logger();

    let p = Path::new("src/data/day1.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);

    let mut sum: u32 = 0;
    let mut first_digit: u32;
    let mut last_digit: u32 = 0;
    let mut digit: u32;

    for (line_idx, line) in reader.lines().enumerate() {
        first_digit = 0;
        for character in line.unwrap().chars() {
            if character.is_digit(10) {
                digit = character.to_digit(10).unwrap();
                last_digit = digit;

                if first_digit == 0 {
                    first_digit = digit;
                }
            }
        }
        log::debug!("Line {line_idx}: first_digit {first_digit} last_digit {last_digit}");
        sum += first_digit * 10 + last_digit;
    }

    log::info!("{}", sum);
}
