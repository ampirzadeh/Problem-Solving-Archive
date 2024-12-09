use std::collections::HashMap;

use advent_of_code_2024::Solution;

pub struct Day8 {
    pub input: String,
}

impl Day8 {
    fn get_numbers(line: &str) -> (i128, i128) {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        (nums[0], nums[1])
    }
}

impl Solution for Day8 {
    fn part1(&self) -> i128 {
        let mut first_set: Vec<i128> = Vec::new();
        let mut second_set: Vec<i128> = Vec::new();

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

    fn part2(&self) -> i128 {
        let mut first_set: Vec<i128> = Vec::new();
        let mut second_set: Vec<i128> = Vec::new();

        for line in self.input.split("\n") {
            let nums = Self::get_numbers(line);

            first_set.push(nums.0);
            second_set.push(nums.1);
        }

        let mut frequency_map: HashMap<i128, i128> = HashMap::new();

        for num in second_set {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        let mut sum = 0;

        for num in first_set {
            sum += num * frequency_map.get(&num).unwrap_or(&0i128);
        }

        sum
    }
}
