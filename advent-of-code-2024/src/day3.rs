use regex::Regex;

use advent_of_code_2024::Solution;

pub struct Day3 {
    pub input: String,
}

impl Day3 {
    fn mul(line: &String) -> i128 {
        // using capture groups in regex was really helpful in 2023 as well, for day 2 for instance
        let re = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

        let mut sum = 0;

        for cap in re.captures_iter(line.as_str()) {
            if let Some(first) = cap.name("first") {
                if let Some(second) = cap.name("second") {
                    sum += first.as_str().parse::<i128>().unwrap()
                        * second.as_str().parse::<i128>().unwrap();
                }
            }
        }

        sum
    }
}

impl Solution for Day3 {
    fn part1(&self) -> String {
        Self::mul(&self.input).to_string()
    }

    fn part2(&self) -> String {
        let mut sum = 0;

        let dont_sections: Vec<&str> = self.input.split("don't()").collect();
        for (i, dont_section) in dont_sections.iter().enumerate() {
            if i == 0 {
                sum += Self::mul(&dont_section.to_string());
                continue;
            }
            if let Some(do_section) = dont_section.split_once("do()") {
                sum += Self::mul(&do_section.1.to_string());
            }
        }

        sum.to_string()
    }
}
