use std::collections::HashMap;

use advent_of_code_2024::Solution;

pub struct Day1 {
    pub input: String,
}

impl Day1 {
    fn get_numbers(line: &str) -> (i32, i32) {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        (nums[0], nums[1])
    }
}

impl Solution for Day1 {
    fn part1(&self) -> i32 {
        let mut first_set: Vec<i32> = Vec::new();
        let mut second_set: Vec<i32> = Vec::new();

        for line in self.input.split("\n") {
            let nums = Self::get_numbers(line);

            first_set.push(nums.0);
            second_set.push(nums.1);
        }

        first_set.sort();
        second_set.sort();

        let mut sum = 0;
        for i in 0..(first_set.len()) {
            sum += (first_set[i]).abs_diff(second_set[i]);
        }

        sum.try_into().unwrap()
    }

    fn part2(&self) -> i32 {
        let mut first_set: Vec<i32> = Vec::new();
        let mut second_set: Vec<i32> = Vec::new();

        for line in self.input.split("\n") {
            let nums = Self::get_numbers(line);

            first_set.push(nums.0);
            second_set.push(nums.1);
        }

        let mut frequency_map: HashMap<i32, i32> = HashMap::new();

        for num in second_set {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        let mut sum = 0;

        for num in first_set {
            sum += num * frequency_map.get(&num).unwrap_or(&0i32);
        }

        sum
    }
}
