use log;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    setupper::logger();

    let p = Path::new("src/data/day2.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);
    let mut sum: u32 = 0;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let re = Regex::new(
        r"(Game (?P<game_id>\d+))|((?P<blue>\d+) blue)|((?P<red>\d+) red)|((?P<green>\d+) green)",
    )
    .unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut invalid = false;
        let mut game_id = 0;

        for cap in re.captures_iter(&line) {
            if let Some(gid) = cap.name("game_id") {
                game_id = gid.as_str().parse::<u32>().unwrap_or(0);
            }
            if let Some(r) = cap.name("red") {
                if r.as_str().parse::<u32>().unwrap_or(0) > max_red {
                    invalid = true;
                    break;
                }
            }
            if let Some(g) = cap.name("green") {
                if g.as_str().parse::<u32>().unwrap_or(0) > max_green {
                    invalid = true;
                    break;
                }
            }
            if let Some(b) = cap.name("blue") {
                if b.as_str().parse::<u32>().unwrap_or(0) > max_blue {
                    invalid = true;
                    break;
                }
            }
        }
        if invalid {
            continue;
        }
        sum += game_id;
    }

    log::info!("{}", sum);
}
