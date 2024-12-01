use std::{
    collections::HashMap,
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

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for num in second_set {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut sum = 0;

    for num in first_set {
        sum += num * frequency_map.get(&num).unwrap_or(&0i32);
    }

    println!("{}", sum);
}
