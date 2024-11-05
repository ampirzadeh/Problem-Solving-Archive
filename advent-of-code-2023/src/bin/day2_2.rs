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
    let mut sum: usize = 0;

    let re = Regex::new(
        r"(Game (?<game_id>\d+))|((?<blue>\d+) blue)|((?<red>\d+) red)|((?<green>\d+) green)",
    )
    .unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut min_red = 1;
        let mut min_green = 1;
        let mut min_blue = 1;

        for cap in re.captures_iter(&line) {
            if let Some(r) = cap.name("red") {
                let r = r.as_str().parse::<usize>().unwrap_or(0);
                if r > min_red {
                    min_red = r;
                }
            }
            if let Some(g) = cap.name("green") {
                let g = g.as_str().parse::<usize>().unwrap_or(0);
                if g > min_green {
                    min_green = g;
                }
            }
            if let Some(b) = cap.name("blue") {
                let b = b.as_str().parse::<usize>().unwrap_or(0);
                if b > min_blue {
                    min_blue = b;
                }
            }
        }

        log::debug!(
            "red{min_red} green{min_green} blue{min_blue} {}",
            min_red * min_green * min_blue
        );
        sum += min_red * min_green * min_blue;
    }

    log::info!("{}", sum);
}
