use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn is_valid(report: &Vec<i32>) -> bool {
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
        let levels: Vec<i32> = report
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if is_valid(&levels) {
            valid_counter += 1;
            continue;
        }

        let mut sublevels: Vec<Vec<i32>> = Vec::new();
        for i in 0..levels.len() {
            let subset: Vec<i32> = levels
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &value)| value)
                .collect();

            sublevels.push(subset);
        }
        for sublevel in sublevels {
            if is_valid(&sublevel) {
                valid_counter += 1;
                break;
            }
        }
    }

    println!("{}", valid_counter);
}
