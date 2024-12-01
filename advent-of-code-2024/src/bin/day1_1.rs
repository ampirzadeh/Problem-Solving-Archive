use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let p = Path::new("src/data/day1.txt");
    let file = File::open(&p).unwrap();
    let reader = BufReader::new(&file);

    let mut first_set: Vec<i32> = Vec::new();
    let mut second_set: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let nums = line
            .unwrap()
            .split("   ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        first_set.push(nums[0]);
        second_set.push(nums[1]);
    }

    first_set.sort();
    second_set.sort();

    let mut sum = 0;
    for i in 0..(first_set.len()) {
        sum += (first_set[i]).abs_diff(second_set[i]);
    }

    println!("{}", sum);
}
