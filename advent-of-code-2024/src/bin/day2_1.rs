use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn is_valid(report: Vec<i32>) -> bool {
    let mut differences: Vec<i32> = Vec::new();

    for i in 1..report.len() {
        let first = report[i - 1];
        let second = report[i];

        differences.push(first - second);
    }

    differences.iter().all(|&x| x.abs() >= 1 && x.abs() <= 3)
        && (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0))
}

fn main() {
    let p = Path::new("src/data/day2.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);

    let mut valid_counter = 0;

    for line in reader.lines() {
        let report = line.unwrap();
        if is_valid(
            report
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        ) {
            valid_counter += 1;
        }
    }

    println!("{}", valid_counter);
}
