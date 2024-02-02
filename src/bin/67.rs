use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // ! Provide file
    let file = File::open("./src/bin/tree.txt").unwrap();
    let reader = BufReader::new(file);

    let mut triangle: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<u64> = Vec::new();
        line.split(" ")
            .for_each(|x| row.push(x.parse::<u64>().unwrap()));
        triangle.push(row);
    }

    let mut m;
    for row in (0..triangle.len()).rev() {
        for col in 0..triangle[row].len() - 1 {
            m = cmp::max(triangle[row][col], triangle[row][col + 1]);
            triangle[row - 1][col] += m;
        }
    }

    println!("{}", triangle[0][0]);
}
