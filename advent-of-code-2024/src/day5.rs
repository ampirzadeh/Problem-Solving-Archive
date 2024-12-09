use std::collections::HashMap;

use advent_of_code_2024::Solution;

pub struct Day5 {
    pub input: String,
}

impl Day5 {
    fn get_precedence_rules(&self) -> HashMap<&str, Vec<&str>> {
        // 47 -> [53, 13, 61]
        let mut precedence_rules: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in self.input.split('\n') {
            if let Some((n1, n2)) = line.split_once('|') {
                precedence_rules
                    .entry(n1)
                    .and_modify(|x| {
                        x.push(n2);
                    })
                    .or_insert(vec![n2]);
            } else {
                break;
            }
        }

        precedence_rules
    }

    fn is_valid_duo(&self, n1: &str, n2: &str) -> bool {
        if let Some(must_come_before) = self.get_precedence_rules().get(n2) {
            return !must_come_before.contains(&n1);
        }
        true
    }

    fn is_valid_nums(&self, nums: &Vec<&str>) -> bool {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if !self.is_valid_duo(nums[i], nums[j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Solution for Day5 {
    fn part1(&self) -> i128 {
        let mut valid_sum: i128 = 0;

        for line in self.input.split('\n') {
            if line.is_empty() || line.contains('|') {
                continue;
            }

            let nums: Vec<&str> = line.split(",").collect();
            if self.is_valid_nums(&nums) {
                valid_sum += nums[(nums.len() - 1) / 2].parse::<i128>().unwrap();
            }
        }

        valid_sum
    }

    fn part2(&self) -> i128 {
        let mut valid_sum: i128 = 0;

        for line in self.input.split('\n') {
            if line.is_empty() || line.contains('|') {
                continue;
            }

            let mut nums: Vec<&str> = line.split(",").collect();
            if self.is_valid_nums(&nums) {
                continue;
            }

            loop {
                let mut should_continue = false;

                for i in 0..nums.len() {
                    for j in (i + 1)..nums.len() {
                        if !self.is_valid_duo(nums[i], nums[j]) {
                            should_continue = true;
                            nums.swap(i, j); // bubble sort-ish
                        }
                    }
                }

                if should_continue == false {
                    valid_sum += nums[(nums.len() - 1) / 2].parse::<i128>().unwrap();
                    break;
                }
            }
        }

        valid_sum
    }
}
